from gtts import gTTS
import os

def speak(text, lang="en", tld=""):
    tts = gTTS(text=text, lang=lang, tld=tld)
    tts.save("__text_output.mp3")
    os.system("ffplay -v 0 -nodisp -autoexit __text_output.mp3")

if __name__ == "__main__":
    speak(input(), "fr", "fr")