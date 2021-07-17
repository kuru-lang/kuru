```aratar
# 16-Bit Signed Year AD Starting At Julian Day.
def Year(Int[-4_712 ~ 60_000])

# 4-Bit one of the twelve months
def Month(Int[1 ~ 12])

# 16-Bit A day of the year.
def Day(Month, Int[1 ~ 31])

# 32-Bit date.
def Date(Year, Day)

# 32-Bit nanosecond within a second.
def Nanosec(Int[0 ~ 999_999_999])

# 32-Bit Second within a year.
def Second(Int[0 ~ 31_622_400])

# 32-Bit Time of Day in 1/100ths of seconds (Use for timestamps)
def Time(Int[0 ~ 3_162_240_000])

# 64-Bit Instant with Nanosecond Precision (starts at 2020, goes for ~5.5k yrs).
# Wraps after that.  Use for high-precision timing of how long something takes.
def Instant(Int[0 ~ 18_446_744_073_709_551_615])

# 64-Bit Signed Duration with Nanosecond Precision.
def Duration(Int[-9_223_372_036_854_775_808 ~ 9_223_372_036_854_775_807])

# 32-Bit Instant with Microsecond Precision (start unspecified, goes for ½ day).
# Wraps after that. Use for medium-precision timing of how long something takes.
def Inst(Int[0 ~ 4_294_967_295])

# 32-Bit Unsigned Duration with Microsecond Precision.
def Dur(Int[0 ~ 4_294_967_295])

# 4-Bit day of the week.
def Weekday: ${
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

#### Time Units ####

# Unit of 1 second
def Seconds[D](Int[D])
# Unit of 1/1_000 second
def Millis[D](Int[D])
# Unit of 1/1_000_000 second
def Micros[D](Int[D])
# Unit of 1/1_000_000_000 second
def Nanos[D](Int[D])

#### Functions ####

def now() -> Instant {
    let set: {1, 2, 3}
    let result: {let @a: 1 + 2; a +: 5; a}
    ##
    match var_a {
        2 { ## }
        1 { ## }
    } else match @var_b {
        VariantA('x, 'y, 'z) { ## }
        VariantB('a: 1 | 2) { ## }
        VariantB(@a: None) { ## }
    } else { ## }
}
```
