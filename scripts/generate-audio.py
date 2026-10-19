from pathlib import Path
import math
import random
import struct
import wave

output = Path(__file__).resolve().parents[1] / "assets/audio/wind.wav"
output.parent.mkdir(parents=True, exist_ok=True)
rate = 22050
random.seed(57)
samples = bytearray()
brown = 0.0
duration = 12
for index in range(rate * duration):
    time = index / rate
    brown = brown * 0.965 + random.uniform(-1, 1) * 0.035
    wind = brown * (0.65 + 0.15 * math.sin(time * math.tau / duration))
    engine = math.sin(time * math.tau * 55) * 0.025 + math.sin(time * math.tau * 110) * 0.01
    pad = sum(math.sin(time * math.tau * tone) for tone in (110, 165, 220)) * 0.007
    fade = min(1.0, index / 600, (rate * duration - index) / 600)
    sample = max(-1, min(1, (wind + engine + pad) * fade))
    samples.extend(struct.pack("<h", int(sample * 32767)))
with wave.open(str(output), "wb") as audio:
    audio.setnchannels(1)
    audio.setsampwidth(2)
    audio.setframerate(rate)
    audio.writeframes(samples)
