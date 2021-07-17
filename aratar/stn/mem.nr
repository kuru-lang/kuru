"""
This is a basic type with 2 variants that optionally wraps another type.
"""
opt enum: no [    # `opt` defaults to `no`.
      no      : 0 # The variant of `opt` that hold nothing.
      yes ?var: _ # The variant of `opt` that optionally holds data.
]

"""
The `any` type wraps other type(s) like a union.
# Example
```
rgba8 struct [r u8: 0 # red # g u8: 0 # green # b u8: 0 # blue # a u8: 0 # alpha #]
rgba16 struct [
      r u16: 0 # red
      g u16: 0 # green
      b u16: 0 # blue
      a u16: 0 # alpha
]
hsva16 struct [
      h u16: 0 # hue
      s u16: 0 # saturation
      v u16: 0 # value
      a u16: 0 # alpha
]
color type: any rgba8 rgba16 hsva16
color var := color.rgba8[r:255 g:255 b:255 a:255]
match color [
      rgba8 : txt(rgba8)
      rgba16: "unreachable", !!
      hsva16: "unreachable", !!
]
```
"""
any enum [
      {var}
]
