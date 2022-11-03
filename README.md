<h1 align="center" style="border-bottom: none; margin-bottom: 0;">
	Rusty Libft: 
</h1>
<h2 align="center">
A Reimplementation of 42's Common Core Libft in Rust, to learn Rust
</h2>

```
                 R RR RR                  Jolan "Blenderwizard" Rathelot
              R RRRRRRRR R          R     ------------------------------
 R RR       R RRRRRRRRRRRRR R      RR     Project: Rusty libft
rR RRR    R RRRRRRRRRRRRRRRRR R   RRR R   Created: October 30th
RRR RR   RRRRRRRRRRRRRRRRRRRRRRR  RRRRR   Language: Rust (100.0 %)
 RRRRR  RRRRRRRRRRRRRRRRRRRRRRRR  RRRR    Lines of code: 776
  RRR RRRRRRRRRRRRRRRRRRRRRRRRRRRR RR     Size: 27.67 KiB
    R  RRRRRRRRRR=  RR=  RRRRRRRRRRR
     RRRRRRRRRRRR=  RR=  RRRRRRRRRR       
      RRRRRRRRRRR   RR   RRRRRRRRRR       
     RR==RRRRRRRRRRRRRRRRRRRRRR===RR      
     RR =  ==RRRRRRR  RRRRRR==  = RR      
      RR =     ===========     = RR      
       RR                        R      
        R                       R        
         R                                 
```

This project was to introduce myself to how Rust works using the first project given to us as 42 students, reimplementing some libc functions + a few others. 

I quickly realised that due to Rusts more Memory Safe approach, alot of these functions become ilogical or imposible to implement (such as strlcat).

I tried to follow the same limitations that the libft project gives us, such as limiting memory allocation in fuctions that don't use it in the libc. However I folowed the originial limitations of the project and their c implementations loosly.

I also used this project to learn how Rust documentation and testing works.

``` bash
# To build the libary
cargo build -r

# To test the library
cargo t -r

# To build documentation
cargo doc -r 
```