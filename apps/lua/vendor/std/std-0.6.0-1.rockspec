package = 'std'
version = '0.6.0-1'
rockspec_format = '3.0'
description = {
    summary = 'Optional support library provided with Temper',
    detailed = 'Authors: Temper Contributors',
    homepage = 'https://temperlang.dev/',
    license = 'Apache-2.0 OR MIT',
}
build = {
    type = 'builtin',
    copy_directories = {'.'},
    modules = {},
}
dependencies = {
    'lua >= 5.1',
    'temper-core == 0.6.0',
}
source = {
    url = 'file:///Users/robertgrayson/code/generic_temper_orm/temper.out/lua/std/std.zip',
}
