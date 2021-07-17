# Input / Output
## All Functions
```
mono!()         # Play mono audio through speaker(s)
stereo!()       # Play stereo audio through speakers
surround!()     # Play surround sound audio through speakers
speakers!()     # Play audio through custom speaker setup
microphones!()  # Record audio through microphone(s)
info!()         # Write text to info log file specific to the current thread
warn!()         # Write text to warn log file specific to the current thread
debug!()        # Write text to debug log file specific to the current thread
cameras!()      # Record pictures from phone camera(s) and/or webcam(s)
screens!()      # Display pictures on phone/computer screen(s)
```

## Sound
```aratar
# Send AudioSamples to speaker(s) asynchronously (non-blocking)
#
# This function takes a closure, the closure is run with a buffer the size of
# the number of samples needed to send to the speaker.  
mono!(run: Func(buffer @[MonoSample]))
stereo!(run: Func(buffer @[StereoSample]))
surround!(run: Func(Buffer @[SurroundSample]))
speakers!(run: Func(Buffer @[@[MonoSample]]))
# Receive AudioSamples from microphones
#
# This function takes a closure, the closure is run with a buffer the size of
# the number of samples received from all microphones.  For each index in the
# buffer there is a slice of samples for each microphone at that instant.
# Stereo microphones are split into left and right (treated as 2 microphones).
microphones!(run: Func(buffer [[Int16]]))
```

## Files
```aratar
info!(text Text)
warn!(text Text)
debug!(text Text)
```

## Picture
```aratar
# Send pixel data to display(s)
screens!(picture: Func(@[Picture]))
# Receive pixel data (frame(s)) from camera.
cameras!(picture: Func([Picture]))
```

## Graphical Processor
TODO

```aratar
# Send picture to GPU.
gpu.send!(picture: Picture) -> TexCoords
# Append vertex data to GPU.
gpu.send!([Float32x4])
# Send model index data to GPU.
gpu.send!([Uint32]) -> Model


# Receive rendered picture from GPU.
gpu.recv!() -> Picture
```
