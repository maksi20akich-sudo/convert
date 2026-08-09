// convert.cs
using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using SixLabors.ImageSharp;
using SixLabors.ImageSharp.PixelFormats;
using SixLabors.ImageSharp.Processing;
using SixLabors.ImageSharp.Formats.Jpeg;

class Convert
{
    static void ConvertPngToJpg(string inputPath, string outputPath, int quality, int resizeW, int resizeH)
    {
        using (var image = Image.Load<Rgba32>(inputPath))
        {
            // Конвертация в RGB (JPG не поддерживает альфа)
            var rgbImage = image.CloneAs<Rgb24>();
            // Изменение размера
            if (resizeW > 0 && resizeH > 0)
            {
                rgbImage.Mutate(x => x.Resize(resizeW, resizeH));
            }
            // Сохранение
            var encoder = new JpegEncoder
            {
                Quality = quality,
                ColorType = JpegColorType.YCbCr,
                ChromaSubsampling = JpegChromaSubsampling.Ratio420
            };
            Directory.CreateDirectory(Path.GetDirectoryName(outputPath));
            rgbImage.Save(outputPath, encoder);
        }
    }

    static void ProcessFiles(List<string> inputs, int quality, int resizeW, int resizeH, string outputDir, bool overwrite, bool recursive)
    {
        var files = new List<string>();
        foreach (var item in inputs)
        {
            if (File.Exists(item) && item.ToLower().EndsWith(".png"))
            {
                files.Add(item);
            }
            else if (Directory.Exists(item))
            {
                var search = recursive ? SearchOption.AllDirectories : SearchOption.TopDirectoryOnly;
                files.AddRange(Directory.GetFiles(item, "*.png", search));
            }
            else if (item.Contains("*"))
            {
                // упрощённо — маски не обрабатываем
            }
        }
        if (files.Count == 0)
        {
            Console.WriteLine("Не найдено PNG-файлов.");
            return;
        }
        Directory.CreateDirectory(outputDir);
        int total = files.Count;
        Console.WriteLine($"Найдено {total} PNG-файлов.");
        for (int i=0; i<total; i++)
        {
            var inputFile = files[i];
            var outName = Path.GetFileNameWithoutExtension(inputFile) + ".jpg";
            var outPath = Path.Combine(outputDir, outName);
            if (File.Exists(outPath) && !overwrite)
            {
                Console.WriteLine($"[{i+1}/{total}] {outPath} уже существует, пропуск.");
                continue;
            }
            Console.WriteLine($"[{i+1}/{total}] Конвертация {inputFile} -> {outPath}");
            try
            {
                ConvertPngToJpg(inputFile, outPath, quality, resizeW, resizeH);
            }
            catch (Exception e)
            {
                Console.WriteLine($"  Ошибка при конвертации {inputFile}: {e.Message}");
            }
        }
        Console.WriteLine("Готово!");
    }

    static void Main(string[] args)
    {
        if (args.Length == 0)
        {
            Console.WriteLine("Использование: dotnet run <PNG-файлы/папки> [--quality N] [--resize ШxВ] [--output DIR] [--overwrite] [--recursive]");
            return;
        }
        var inputs = new List<string>();
        int quality = 85;
        int resizeW = 0, resizeH = 0;
        string outputDir = ".";
        bool overwrite = false;
        bool recursive = false;
        for (int i=0; i<args.Length; i++)
        {
            switch (args[i])
            {
                case "--quality":
                    if (i+1 < args.Length) quality = int.Parse(args[++i]);
                    break;
                case "--resize":
                    if (i+1 < args.Length)
                    {
                        var s = args[++i];
                        var parts = s.Split('x');
                        if (parts.Length == 2)
                        {
                            resizeW = int.Parse(parts[0]);
                            resizeH = int.Parse(parts[1]);
                        }
                    }
                    break;
                case "--output":
                    if (i+1 < args.Length) outputDir = args[++i];
                    break;
                case "--overwrite":
                    overwrite = true;
                    break;
                case "--recursive":
                    recursive = true;
                    break;
                default:
                    inputs.Add(args[i]);
                    break;
            }
        }
        ProcessFiles(inputs, quality, resizeW, resizeH, outputDir, overwrite, recursive);
    }
}
