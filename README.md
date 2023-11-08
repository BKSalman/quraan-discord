<h1 dir="rtl">بسم الله الرحمن الرحيم<h1/>

# Qura'an discord

A simple discord bot for requesting Suar (سور), Ayat (آيات), pages, and tafseer (تفسير) of the Qura'an

# Running

first of all you need to convert the pdf `arabic-quran.pdf` in the `data` folder by using `pdfimages`:
```bash
pdfimages -png data/arabic-quran.pdf data/arabic-quran-images/
```

this will extract the mushaf pages images from the pdf

then you can run the discord bot with cargo:
```bash
cargo run
# or in release mode
cargo run --release
```

# Sources
The Mushaf used (`data/arabic-quran.pdf`) is the Al Madinah Al Munawarah (Hafs Narration) Mushaf from King Fahad Glorious Qura'an Printing Complex

