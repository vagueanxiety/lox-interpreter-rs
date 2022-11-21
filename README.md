# lox-interpreter-rs
Yet another [Lox](https://craftinginterpreters.com/the-lox-language.html) interpreter.

```
>> print lox();

   ,--,                                
,---.'|       ,----..                  
|   | :      /   /   \  ,--,     ,--,  
:   : |     /   .     : |'. \   / .`|  
|   ' :    .   /   ;.  \; \ `\ /' / ;  
;   ; '   .   ;   /  ` ;`. \  /  / .'  
'   | |__ ;   |  ; \ ; | \  \/  / ./   
|   | :.'||   :  | ; | '  \  \.'  /    
'   :    ;.   |  ' ' ' :   \  ;  ;     
|   |  ./ '   ;  \; /  |  / \  \  \    
;   : ;    \   \  ',  /  ;  /\  \  \   
|   ,/      ;   :    / ./__;  \  ;  \  
'---'        \   \ .'  |   : / \  \  ; 
              `---`    ;   |/   \  ' | 
                       `---'     `--`  

>> print 1+1;
2
>> print clock();
1668899599.0131063
>> 
```

This is just for myself to learn Rust while reading the book "[Crafting Interpreters](https://craftinginterpreters.com/)" by Robert Nystrom.

## Usage
```
# start a REPL
cargo run

# or run a Lox file (-r for release mode)
cargo run -- path/to/foobar.lox

# run tests
cargo test
```

## Caveats
- Current implementation handles errors slightly differently from the Java
  version
- No optimizations have been done and the performance is not great
