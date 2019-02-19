# RASCII
### ASCII image renderer written in Rust

Rascii allows you to preview image files in terminal.
By default it shows the image in very low resolution.

```./rascii imgs/1.png```

![](https://raw.githubusercontent.com/mightykho/rascii/master/readme/1.png)


By tweaking some values you can get much better results.

```./rascii imgs/1.png -w 260```

![](https://raw.githubusercontent.com/mightykho/rascii/master/readme/2.png)


To get colourful results pass `-c` argument (Currently doesn't work in Mac OS
terminal.)

```./rascii imgs/1.png -w 260 -c```

![](https://raw.githubusercontent.com/mightykho/rascii/master/readme/3.png)


Rascii also allows you to play GIF animations (only 256 colour gifs are supported)

```./rascii gifs/5.gif -w 460 -f 6 -p 3```

[![Watch the video](https://raw.githubusercontent.com/mightykho/rascii/master/readme/4.png)](https://youtu.be/RAfJnAe2HhA)


Gif animations could be also previewed in colour but it is much less performant
than ASCII version.

![](https://raw.githubusercontent.com/mightykho/rascii/master/readme/5.gif)


## Usage
```
USAGE:
    rascii [FLAGS] [OPTIONS] <IMAGE>

FLAGS:
    -c, --color      Use colors instead of symbols. Less performant when used with GIFs
    -h, --help       Prints help information
    -i, --invert     Inverts image. Useful for light-themed terminals
    -V, --version    Prints version information

OPTIONS:
    -f, --fps <FPS>                Sets frames per second. Default: 2
    -p <PIXEL ASPECT RATIO>        Sets aspect ratio of a symbol-pixel. Might need tweaking depending on line height.
                                   Default: 3.0
    -w <IMAGE WIDTH>               Sets image width in symbols. Default: 100

ARGS:
    <IMAGE>    Image path to preview
```
