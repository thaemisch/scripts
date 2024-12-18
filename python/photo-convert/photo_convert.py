import os
from PIL import Image

def convert_to_square(image_path, output_path, padding=250):
    # Open the input image
    image = Image.open(image_path)
    
    # Calculate the size of the new square image
    max_side = max(image.size)
    new_size = (max_side + 2 * padding, max_side + 2 * padding)
    
    # Create a new image with a white background
    new_image = Image.new("RGB", new_size, (255, 255, 255))
    
    # Calculate the position to paste the original image
    paste_position = (
        (new_size[0] - image.size[0]) // 2,
        (new_size[1] - image.size[1]) // 2
    )
    
    # Paste the original image onto the new image
    new_image.paste(image, paste_position)
    
    # Save the new image
    new_image.save(output_path)

def convert_directory(input_dir, output_dir, padding=250):
    if not os.path.exists(output_dir):
        os.makedirs(output_dir)
    
    for filename in os.listdir(input_dir):
        if filename.lower().endswith(".jpg"):
            input_path = os.path.join(input_dir, filename)
            output_path = os.path.join(output_dir, filename)
            convert_to_square(input_path, output_path, padding)

# Example usage
convert_directory("input", "output")