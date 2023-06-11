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

def GetProjectTree(): void
	# first get tree of project
	var rt = ""
	if expand('%:f') == "pom.xml"
		rt = system(filename .. " -q " .. expand('%:p:h') .. "/src/main/java/")
	else
		rt = system(filename .. " -q " .. expand('%:p'))
	endif
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
	popup_menu(classs, {'callback': 'OpenFile', 'line': 3, 'col': 3, 'minwidth': 60, 'maxheight': 12, 'borderchars': ['─', '│', '─', '│', '┌', '╮', '╯', '╰']})
enddef

var new_class_name = ""

def CreateClass(): void
	new_class_name = system(filename .. " -p " .. expand('%:p'))
	popup_menu(new_class_name, {
				callback: (_, result) => {
					if result == 1
						var flag = system(filename .. " -c " .. new_class_name .. " " .. expand('%:p'))
						if flag != "err"
							exec "edit " .. flag
						endif
					endif
				},
				filter: (id, key) => {
					if key == "\<BS>"
						new_class_name = new_class_name[: -2]
					elseif key =~ '[a-zA-Z0-9.]'
						new_class_name = new_class_name .. key
					elseif key == "\<C-w>"
						if new_class_name[len(new_class_name) - 1] == "."
							new_class_name = new_class_name[: -2]
						else
							for i in range(len(new_class_name) - 1, 0, -1)
								if new_class_name[i] == "."
									new_class_name = new_class_name[: i]
									break
								elseif i == 0
									new_class_name = ""
								endif
							endfor
						endif
					elseif key == "\<Esc>"
						popup_close(id, -1)
					elseif key == "\<CR>"
						popup_close(id, 1)
					endif
					popup_settext(id, new_class_name)
					return true
				},
				minwidth: 60,
				borderchars: ['─', '│', '─', '│', '╭', '╮', '╯', '╰'],
			})
enddef

command GetProjectTree call GetProjectTree()
command CreateClass call CreateClass()

nnoremap <Leader>kk :GetProjectTree<CR>
nnoremap <Leader>kn :CreateClass<CR>

g:mvnJumperLoaded = 1
