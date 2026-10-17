[parallel]
all: u32-expand u16-expand non-expand plain-expand i128-expand flag-expand friend-expand friendship-expand friendships-expand text-expand u32-stable-const-expand
  just fmt

[group("example")]
friend-expand: (z-expand 'my_friend')
[group("example")]
friend: friend-expand fmt

[group("example")]
friendship-expand: (z-expand 'my_friendship')
[group("example")]
friendship: friendship-expand fmt

[group("example")]
friendships-expand: (z-expand 'my_friendships')
[group("example")]
friendships: friendships-expand fmt

[group("example")]
u32-expand: (z-expand 'my_u32')
[group("example")]
u32: u32-expand fmt

[group("example")]
u16-expand: (z-expand 'my_u16')
[group("example")]
u16: u16-expand fmt

[group("example")]
i128-expand: (z-expand 'my_i128')
[group("example")]
i128: i128-expand fmt

[group("example")]
non-expand: (z-expand 'my_u32_non_const')
[group("example")]
non: non-expand fmt

[group("example")]
flag-expand: (z-expand 'my_flag')
[group("example")]
flag: flag-expand fmt

[group("example")]
plain-expand: (z-expand 'my_plain')
[group("example")]
plain: plain-expand fmt

[group("example")]
text-expand: (z-expand 'my_text')
[group("example")]
text: text-expand fmt

[group("example")]
u32-stable-const-expand: (z-expand 'my_u32_stable_const')
[group("example")]
u32-stable-const: u32-stable-const-expand fmt
