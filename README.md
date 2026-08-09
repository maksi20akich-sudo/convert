🖼️ Конвертер изображений (PNG→JPG) — быстрый и удобный
Версия: 1.0.0 | Лицензия: MIT | Статус: ✅ Активная разработка

https://img.shields.io/github/repo-size/yourusername/image-converter https://img.shields.io/github/last-commit/yourusername/image-converter https://img.shields.io/github/languages/count/yourusername/image-converter

📸 Описание
Конвертер изображений — это консольная утилита для пакетного преобразования изображений из формата PNG в JPG (JPEG). Программа поддерживает:

✅ Конвертацию одного или нескольких PNG-файлов в JPG

✅ Настройку качества сжатия (от 1 до 100)

✅ Изменение размера изображения (опционально)

✅ Пакетную обработку всех PNG-файлов в папке

✅ Прогресс-бар и подробные логи

✅ Перезапись существующих файлов или создание новых

✅ Рекурсивный обход подпапок

Проект содержит 8 полноценных реализаций на разных языках программирования. Все версии используют популярные библиотеки для работы с изображениями и предоставляют единый интерфейс командной строки.

✨ Возможности
Функция	Описание
Конвертация PNG → JPG	Преобразование с сохранением качества
Качество (1–100)	Настройка степени сжатия JPEG
Изменение размера	Пропорциональное или точное изменение ширины/высоты
Пакетная обработка	Конвертация всех PNG-файлов в директории
Прогресс-бар	Отображение хода выполнения
Перезапись	Опция перезаписи существующих файлов
Рекурсивный обход	Обработка PNG во всех подпапках
Кроссплатформенность	Работает на Linux, macOS, Windows
📦 Установка и запуск
Каждая реализация находится в отдельной папке. Для запуска требуется соответствующий компилятор/интерпретатор и библиотеки.

Язык	Файл	Зависимости	Команда запуска
Python	convert.py	Pillow	pip install Pillow && python3 convert.py image.png
Go	convert.go	github.com/disintegration/imaging	go mod init && go get github.com/disintegration/imaging && go run convert.go image.png
Rust	convert.rs	image, glob	cargo add image glob && cargo run -- image.png
C++	convert.cpp	stb_image, stb_image_write (заголовочные)	g++ -std=c++17 -o convert convert.cpp && ./convert image.png
Java	Convert.java	javax.imageio (встроен)	javac Convert.java && java Convert image.png
C#	convert.cs	SixLabors.ImageSharp	dotnet add package SixLabors.ImageSharp && dotnet run image.png
Ruby	convert.rb	mini_magick	gem install mini_magick && ruby convert.rb image.png
Node.js	convert.js	sharp, glob	npm install sharp glob && node convert.js image.png
Примечание: Для всех реализаций доступны общие опции: --quality N (качество), --resize WxH (изменение размера), --output DIR (папка для сохранения), --overwrite (перезапись), --recursive (обработка подпапок).

📂 Структура репозитория
text
.
├── README.md
├── python/
│   └── convert.py
├── go/
│   └── convert.go
├── rust/
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
├── cpp/
│   └── convert.cpp
├── java/
│   └── Convert.java
├── csharp/
│   └── convert.cs
├── ruby/
│   └── convert.rb
└── javascript/
    ├── package.json
    └── convert.js
🎮 Использование
bash
# Конвертация одного файла
convert image.png

# Конвертация с качеством 85%
convert image.png --quality 85

# Конвертация и изменение размера до 800x600
convert image.png --resize 800x600

# Пакетная конвертация всех PNG в папке
convert *.png

# Рекурсивная конвертация всех PNG в папке и подпапках
convert --recursive .

# Сохранение в другую папку
convert image.png --output ./jpg

# Перезапись существующих JPG
convert image.png --overwrite
🛠️ Особенности реализаций
Python – использует Pillow (PIL) – самую популярную библиотеку для работы с изображениями.

Go – imaging – простая и быстрая библиотека для манипуляции изображениями.

Rust – image – мощная библиотека с поддержкой множества форматов.

C++ – stb_image и stb_image_write – легковесные заголовочные библиотеки.

Java – встроенный javax.imageio – кросс-платформенный, без внешних зависимостей.

C# – SixLabors.ImageSharp – современная библиотека для .NET.

Ruby – mini_magick – обёртка для ImageMagick/GraphicsMagick.

Node.js – sharp – высокопроизводительная библиотека на основе libvips.

🤝 Вклад
PR и issues приветствуются. Добавляйте поддержку других форматов, улучшайте производительность, расширяйте функциональность.

📄 Лицензия
MIT License.
