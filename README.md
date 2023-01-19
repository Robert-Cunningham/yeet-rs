# 🚨 Yeet 🚨

`yeet <PORT>` kills any process on `<PORT>`. Yeet works on Linux and MacOS.

## Quick start


### Example 1
You're trying to start your server, but it won't start because something else is already using port 3000. 
```
yeet 3000
```

### Example 2
You're trying to start your docker container, but it won't start because something else is already using port 80. 

```
yeet 80
```

### Example 3
You're trying to start your `$IMPORTANT_THING`, but it won't start because `$SOMETHING_ANNOYING` is already using port `$PORT`. 

```
yeet $PORT
```



## Installation
### Shameful one line installation

(you can install `yeet` this way as long as you agree to feel bad about it all day)
```
sudo /bin/bash -c "$(curl -fsSL https://yeet.today/install.sh)"
```

### If you have `cargo` installed
```
cargo install yeet-rs
```

### Normal install
```
$ git clone https://github.com/Robert-Cunningham/yeet-rs
$ cd yeet-rs
$ cargo build --release
$ ./target/release/yeet --version
```

## FAQ

### Why yeet instead of `sudo kill -9 $(sudo lsof -t -i:3000)`?
You should probably use that other command instead. But if you're lazy and can't remember it without stack exchange, `yeet` will get the job done.

### Why would I ever use yeet? Shouldn't this be like a two line shell script?
Yes.

### Why would I ever use yeet? Can't I just alias that lsof command instead?
Yes.

### Why is yeet written in Rust? Wouldn't $OTHER_LANGUAGE be better?
Yes.

### Aren't you a horrible person for suggesting that anyone pipe curl into sudo sh?
Yes.

### Does yeet support any other options?
`--version` and `--help`, but only because it was more annoying to remove them than to leave them in.

## Acknowledgements
The authors would like to thank [Terry](https://www.youtube.com/watch?v=2Bjy5YQ5xPc) for her contribution to humanity.
