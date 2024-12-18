;----Google----------------------------------------------------------------------------------------

;----Search--Win+G
#g::
{
 Send, ^c
 Sleep 50
 Run, https://www.google.com/search?q=%clipboard%
 Return
}

;----Translate--Win+T
#t::
{
 Send, ^c
 Sleep 50
 Run, https://translate.google.com/?sl=auto&tl=en&text=%clipboard%
 Return
}

;----Windows---------------------------------------------------------------------------------------

;----AlwaysOnTop--Win+Shift+T
#+t::WinSet, AlwaysOnTop, Toggle, A
return

;----ShareXScreenshot--Win+S
#s::Send, "+{PrintScreen}"
return
