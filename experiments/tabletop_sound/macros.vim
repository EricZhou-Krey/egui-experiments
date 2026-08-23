command! -nargs=1 AddTab call AddTab(<f-args>)

function! AddTab(tab_name)
    let l:enum_name = a:tab_name
    let l:mod_name = tolower(a:tab_name)

    let l:base_dir = 'experiments/tabletop_sound/src/'

    let l:new_file = l:base_dir . 'tabs/' . l:mod_name . '.rs'
    let l:template = [
        \ 'use crate::state::TTSState;',
        \ '',
        \ 'pub fn ' . l:mod_name . '_title(_state: &mut TTSState) -> egui::WidgetText {',
        \ '    "' . l:enum_name . '".into()',
        \ '}',
        \ '',
        \ 'pub fn ' . l:mod_name . '_ui(_state: &mut TTSState, ui: &mut egui::Ui) {',
        \ '    ui.centered_and_justified(|ui| ui.heading("' . l:enum_name . '"));',
        \ '}'
        \ ]
    call writefile(l:template, l:new_file)

    let l:mod_file = l:base_dir . 'tabs/mod.rs'
    call writefile(['pub mod ' . l:mod_name . ';'], l:mod_file, 'a')

    execute 'edit ' . l:base_dir . 'tab.rs'

    call search('use crate::tabs::{')
    call append('.', '    ' . l:mod_name . '::{' . l:mod_name . '_title, ' . l:mod_name . '_ui},')

    call search('pub enum Tab {')
    call search('}', 'W') 
    call append(line('.') - 1, '    ' . l:enum_name . ',')

    call search('pub const ALL: &''static \[Tab\] = &\[')
    call search('\];', 'W') 
    call append(line('.') - 1, '        Tab::' . l:enum_name . ',')

    call search('pub fn title(')
    call search('match self {')
    call search('}', 'W')
    call append(line('.') - 1, '            Tab::' . l:enum_name . ' => ' . l:mod_name . '_title(state),')

    call search('pub fn ui(')
    call search('match self {')
    call search('}', 'W')
    call append(line('.') - 1, '            Tab::' . l:enum_name . ' => ' . l:mod_name . '_ui(state, ui),')

    write
endfunction


command! -nargs=1 RemoveTab call RemoveTab(<f-args>)

function! RemoveTab(tab_name)
    let l:enum_name = a:tab_name
    let l:mod_name = tolower(a:tab_name)
    let l:base_dir = 'experiments/tabletop_sound/src/'

    let l:target_file = l:base_dir . 'tabs/' . l:mod_name . '.rs'
    
    if delete(l:target_file) == 0
        echom "Deleted " . l:target_file
    else
        echom "Note: " . l:target_file . " was already deleted or not found."
    endif

    execute 'edit ' . l:base_dir . 'tabs/mod.rs'
    execute 'silent! g/^pub mod ' . l:mod_name . ';/d'
    write

    execute 'edit ' . l:base_dir . 'tab.rs'

    execute 'silent! g/^\s*' . l:mod_name . '::{' . l:mod_name . '_title, ' . l:mod_name . '_ui},/d'

    execute 'silent! g/^\s*' . l:enum_name . ',$/d'

    execute 'silent! g/^\s*Tab::' . l:enum_name . ',/d'

    execute 'silent! g/^\s*Tab::' . l:enum_name . ' => ' . l:mod_name . '_title(state),/d'

    execute 'silent! g/^\s*Tab::' . l:enum_name . ' => ' . l:mod_name . '_ui(state, ui),/d'

    write
endfunction

command! -nargs=1 AddMapTool call AddMapTool(<f-args>)

function! AddMapTool(tool_name)
    let l:enum_name = a:tool_name
    let l:const_name = 'MAP_' . toupper(a:tool_name) . '_ICON'
    let l:field_name = tolower(substitute(a:tool_name, '\(\l\)\(\u\)', '\1_\2', 'g')) . '_tool'
    let l:base_dir = 'experiments/tabletop_sound/src/'

    execute 'edit ' . l:base_dir . 'style_sheet.rs'
    call cursor(1, 1)
    call search('pub const MAP_.*_ICON')
    call append(line('.'), 'pub const ' . l:const_name . ': &str = "' . strpart(a:tool_name, 0, 1) . '";')
    write

    execute 'edit ' . l:base_dir . 'style.rs'

    call cursor(1, 1)
    call search('crate::style_sheet::{')
    let l:style_line = getline('.')
    let l:style_line = substitute(l:style_line, 'crate::style_sheet::{', 'crate::style_sheet::{' . l:const_name . ', ', '')
    call setline('.', l:style_line)

    call cursor(1, 1)
    call search('pub struct MapIcons {')
    call search('}', 'W')
    call append(line('.') - 1, '    pub ' . l:field_name . ': &''static str,')

    call cursor(1, 1)
    call search('icons: MapIcons {')
    call search('}', 'W')
    call append(line('.') - 1, '                ' . l:field_name . ': ' . l:const_name . ',')
    write

    execute 'edit ' . l:base_dir . 'tabs/mapview.rs'

    call cursor(1, 1)
    call search('style_sheet::{')
    let l:style_line = getline('.')
    let l:style_line = substitute(l:style_line, 'style_sheet::{', 'style_sheet::{' . l:const_name . ', ', '')
    call setline('.', l:style_line)

    call cursor(1, 1)
    call search('pub enum MapTool {')
    call search('}', 'W')
    call append(line('.') - 1, '    ' . l:enum_name . ',')

    call cursor(1, 1)
    call search('pub const ALL: &''static \[MapTool\] =')
    call search('\];', 'W')
    call append(line('.') - 1, '        MapTool::' . l:enum_name . ',')

    call cursor(1, 1)
    call search('pub fn interact(')
    call search('match state\.map_state\.map_tool {')
    call search('}', 'W')
    call append(line('.') - 1, '            MapTool::' . l:enum_name . ' => {}')

    call cursor(1, 1)
    call search('pub fn icon(')
    call search('match self {')
    call search('}', 'W')
    call append(line('.') - 1, '            MapTool::' . l:enum_name . ' => ' . l:const_name . '.into(),')

    write
endfunction


command! -nargs=1 RemoveMapTool call RemoveMapTool(<f-args>)

function! RemoveMapTool(tool_name)
    let l:enum_name = a:tool_name
    let l:const_name = 'MAP_' . toupper(a:tool_name) . '_ICON'
    let l:field_name = tolower(substitute(a:tool_name, '\(\l\)\(\u\)', '\1_\2', 'g')) . '_tool'
    let l:base_dir = 'experiments/tabletop_sound/src/'

    execute 'edit ' . l:base_dir . 'style_sheet.rs'
    execute 'silent! g/^pub const ' . l:const_name . ': &str/d'
    write

    execute 'edit ' . l:base_dir . 'style.rs'

    execute 'silent! g/^\s*pub ' . l:field_name . ': &''static str,$/d'
    execute 'silent! g/^\s*' . l:field_name . ': ' . l:const_name . ',$/d'
    execute 'silent! %s/,\s*\<' . l:const_name . '\>//ge'
    execute 'silent! %s/\<' . l:const_name . '\>\s*,//ge'
    execute 'silent! %s/\<' . l:const_name . '\>//ge'
    write

    execute 'edit ' . l:base_dir . 'tabs/mapview.rs'

    execute 'silent! g/^\s*' . l:enum_name . ',$/d'
    execute 'silent! g/^\s*MapTool::' . l:enum_name . ',$/d'
    execute 'silent! g/^\s*MapTool::' . l:enum_name . ' => {}.*$/d'
    execute 'silent! g/^\s*MapTool::' . l:enum_name . ' => ' . l:const_name . '\.into(),$/d'

    execute 'silent! %s/,\s*\<' . l:const_name . '\>//ge'
    execute 'silent! %s/\<' . l:const_name . '\>\s*,//ge'
    execute 'silent! %s/\<' . l:const_name . '\>//ge'

    execute 'silent! %s/,\s*MapTool::' . l:enum_name . '\>//ge'
    execute 'silent! %s/MapTool::' . l:enum_name . '\>\s*,//ge'
    execute 'silent! %s/MapTool::' . l:enum_name . '\>//ge'

    write
endfunction
