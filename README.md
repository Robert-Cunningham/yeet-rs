# 🚨🚨🚨 Yeet 🚨🚨🚨

`yeet <PORT>` kills any process on `<PORT>`.

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
$ ./target/release/yeet 3000
```


## Acknowledgements
The authors would like to thank [Terry](https://www.youtube.com/watch?v=2Bjy5YQ5xPc) for her contribution to humanity.
