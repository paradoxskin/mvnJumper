vim9script

if (exists("g:mvnJumperLoaded"))
	finish
endif

var filename =  expand("<sfile>:p:h:h") .. "/target/release/mvnJumper"
var classs = []
var filepath = []

def OpenFile(id: number, res: number)
	if (res != -1)
		exec "edit "  .. filepath[res - 1]
	endif
enddef

def g:GetProjectTree(path: string): void
	# first get tree of project
	var rt = system(filename .. " -q " .. expand('%:p'))
	var lrt = split(rt, ",")
	classs = []
	filepath = []
	for lnn in lrt
		if lnn == ""
			continue
		endif
		var tmpp = split(lnn, "!")
		add(classs, tmpp[0])
		add(filepath, tmpp[1])
	endfor
	# popup windows
	popup_menu(classs, {'callback': 'OpenFile', 'line': 3, 'col': 3, 'minwidth': 60, 'maxheight': 12, 'borderchars': ['─', '│', '─', '│', '╭', '╮', '╯', '╰']})
enddef

def CreateClass(path: string): void
	# create file ok
	# do some init
	# open file
enddef

g:mvnJumperLoaded = 1
