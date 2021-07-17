# AASI (Aratar Application System Interface)
## Notes on "C" Apis
- `[]` Lists are actually two parameters: `(size: Offs, array: Addr)`, in that
  order according to rule 15 of the C Principles.
- `Opt[Addr]` is the same size as `Addr` because `Addr` cannot be `0`.
- `Opt[()]` is stored as web assembly's `i64`: 0=None, 1=Some(())

## Application Exports
```aratar
# Application start
api start() -> ()

### CLOCK ###

# Timer has expired
api aasi_expire(timer: Addr) -> ()

### RANDOM ###

# Random number has finished being generated
api aasi_random(rng: Addr, rand: Data64) -> ()

### FILES ###

# Mutable data from the file system has been loaded.
api aasi_edit(file: Addr, data: @[Data8]) -> ()

# Immutable data from the file system has been loaded.
api aasi_read(file: Addr, data: [Data8]) -> ()

### NETWORKING ###

# Requested packet size has been received from a socket.
api aasi_packet(socket: Addr: data: [Data8]) -> ()

# Other side of socket has hung up.
api aasi_hangup(socket: Addr) -> ()

### AUDIO ###

# Speakers need more audio to play (return stereo or 5.1 surround sound)
api aasi_play(size: Uint64) -> ([Float64x2], Opt[[Float64x4]])

# A Microphone has captured audio
api aasi_capture(microhpone: Addr)

### INPUT ###

# Get input
api aasi_input(input: Input)

### VIDEO ###

# Get picture from webcam
api aasi_picture(picture: Image)

# Write a frame to the screen
api aasi_frame(picture: @Image)
```

## Application Imports
```
### CLOCK ###

# Start a hardware timer.  Returns an opaque memory Address of the timer object,
# which will be passed to `aasi_expire()` when the timer expires.
use aasi_timer(
    secs: Int64[0~],
    nanos: Int32[0~999_999_999]
) -> Addr

### RANDOM ###

# Request random data from the random number generator, to be created in the
# future (when done calls `aasi_random`).
use aasi_rng() -> Addr

### FILES ###

# Lock and open a file (user picks which file)
use aasi_open() -> Addr

# Lock and open a system-chosen-file (one associated with each program)
use aasi_prefs() -> Addr

# Create a new address pointing to the same file.  This allows multiple threads
# to access the same file.
use aasi_share(file: Addr) -> Addr

# Synchronized read followed by possibly writing on a section of a file.
use aasi_sync(file: Addr, byte_offset: Uint64, size: Uint64) -> Opt[Addr]

# Load data from a file non-locking (will abort app if wrong data size).
# When data is loaded into RAM from the hard drive, calls `aasi_load()`.
use aasi_load(file: Addr, byte_offset: Uint64, size: Uint64) -> Opt[Addr]

# Save data to a file non-locking (will abort app if wrong data size on lock)
use aasi_save(section: Addr, data: [Data8]) -> ()

# Edit size of file (may be used to shrink or expand file with zeros, will
# return `None()` if storage device is out of memory)
use aasi_trunc(file: Addr, new_size: Uint64) -> Opt[()]

### NETWORKING ###

# Synchronize, close and unlock a file
use aasi_close(file: Addr) -> ()

# Connect to a socket
use aasi_connect(address: Text) -> Opt[Addr]

# Send data on a socket
use aasi_send(socket: Addr, data: [Data8]) -> ()

# Receive data from a socket
use aasi_recv(socket: Addr, size: Uint64) -> Opt[Addr]

# Disconnect from a socket
use aasi_disconnect(address: Addr)

### AUDIO ###

# Connect to system's default connected speakers
use aasi_speakers()

# Record audio from user-selected source (may be microphone or speakers etc.)
use aasi_record()

### INPUT ###

### VIDEO ###

# Record video from user-selected source (may be screen or webcam etc.)
use aasi_camera() -> Addr

# Request a number of screens
use aasi_screen(number: Uint32) -> [Addr]

# Render onto an image
use aasi_render(image: @Image, cmds: [GpuCmd])

# Use GPU to do computations on floats
use aasi_gpu32(buffer: @[Float32], cmds: [GpuCmd])

# Use GPU to do computations on floats
use aasi_gpu64(buffer: @[Float64], cmds: [GpuCmd])
```
