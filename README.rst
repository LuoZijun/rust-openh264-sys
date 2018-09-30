OpenH264-Sys
===================

.. contents:: 

该库采用静态链接的方式直接链接 `libopenh264.1.8.0.a` ，所以你的系统里面如果装了其它的版本的话，并不会因此而受到一些版本问题的困扰。


前置条件
----------

请确保你的操作系统已经有了以下命令行程序可供使用(大部分系统里面都已经内置):

*   make
*   git
*   cc(gcc/clang)
*   c++(g++/clang++)
*   ar
*   nasm


构建
-----------

.. code:: bash
    
    git clone --recurse-submodules -j8 https://github.com/LuoZijun/rust-openh264-sys.git
    cargo build
    cargo test


使用
-------------

*Cargo.toml*:

.. code:: toml
    
    [dependencies]
    openh264-sys = { git = "https://github.com/LuoZijun/rust-openh264-sys" }


*src/main.rs*:

.. code:: rust
    
    extern crate openh264_sys;

    fn main() {
        let version: OpenH264Version = unsafe { WelsGetCodecVersion() };
        println!("{:?}", version);
    }


高阶接口
----------

更友好的高阶接口: https://github.com/LuoZijun/rust-openh264


相关项目
------------
`saturday06/rust-openh264-sys <https://github.com/saturday06/rust-openh264-sys>`_ , 目前不支持静态链接

