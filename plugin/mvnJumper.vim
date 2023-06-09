vim9script

if (exists("g:mvnJumperLoaded"))
	finish
endif

def GetProjectTree(path: string): void
	" first get tree of project
	" popup windows
	  popup_menu(items, {'line': 3, 'col': 3, 'minwidth': 60, 'maxheight': 12, 'borderchars': ['─', '│', '─', '│', '╭', '╮', '╯', '╰']})
	" q/esc to close
	" j/k to move, enter to jump
enddef

def CreateClass(path: string): void
	" create file ok
	" do some init
	" echo path of the class
enddef

g:mvnJumperLoaded = 1
