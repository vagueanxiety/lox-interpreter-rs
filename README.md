# lox-interpreter-rs
Yet another [Lox](https://craftinginterpreters.com/the-lox-language.html) interpreter.

```
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
1668897738.3461406
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
- Chapter 13 is WIP
- Current implementation handles errors slightly differently from the Java
  version
