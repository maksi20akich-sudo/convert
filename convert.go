// convert.go
package main

import (
	"flag"
	"fmt"
	"image/jpeg"
	"os"
	"path/filepath"
	"strings"
	"sync"

	"github.com/disintegration/imaging"
)

func convertPngToJpg(inputPath, outputPath string, quality int, resize string) error {
	img, err := imaging.Open(inputPath)
	if err != nil {
		return err
	}
	// Изменение размера
	if resize != "" {
		var w, h int
		fmt.Sscanf(resize, "%dx%d", &w, &h)
		if w > 0 && h > 0 {
			img = imaging.Resize(img, w, h, imaging.Lanczos)
		}
	}
	// Сохранение
	out, err := os.Create(outputPath)
	if err != nil {
		return err
	}
	defer out.Close()
	opts := jpeg.Options{Quality: quality}
	return jpeg.Encode(out, img, &opts)
}

func processFiles(inputs []string, quality int, resize, outputDir string, overwrite, recursive bool) {
	var files []string
	for _, item := range inputs {
		info, err := os.Stat(item)
		if err == nil && !info.IsDir() && strings.HasSuffix(strings.ToLower(item), ".png") {
			files = append(files, item)
		} else if err == nil && info.IsDir() {
			if recursive {
				filepath.Walk(item, func(path string, info os.FileInfo, err error) error {
					if err == nil && !info.IsDir() && strings.HasSuffix(strings.ToLower(path), ".png") {
						files = append(files, path)
					}
					return nil
				})
			} else {
				entries, _ := os.ReadDir(item)
				for _, e := range entries {
					if !e.IsDir() && strings.HasSuffix(strings.ToLower(e.Name()), ".png") {
						files = append(files, filepath.Join(item, e.Name()))
					}
				}
			}
		} else if strings.Contains(item, "*") {
			matches, _ := filepath.Glob(item)
			for _, m := range matches {
				if strings.HasSuffix(strings.ToLower(m), ".png") {
					files = append(files, m)
				}
			}
		}
	}
	if len(files) == 0 {
		fmt.Println("Не найдено PNG-файлов.")
		return
	}
	if err := os.MkdirAll(outputDir, 0755); err != nil {
		fmt.Println("Ошибка создания папки:", err)
		return
	}
	total := len(files)
	fmt.Printf("Найдено %d PNG-файлов.\n", total)
	var wg sync.WaitGroup
	sem := make(chan struct{}, 4) // ограничение параллельности
	for i, f := range files {
		wg.Add(1)
		go func(idx int, inputPath string) {
			defer wg.Done()
			sem <- struct{}{}
			defer func() { <-sem }()
			outName := strings.TrimSuffix(filepath.Base(inputPath), ".png") + ".jpg"
			outPath := filepath.Join(outputDir, outName)
			if _, err := os.Stat(outPath); err == nil && !overwrite {
				fmt.Printf("[%d/%d] %s уже существует, пропуск.\n", idx+1, total, outPath)
				return
			}
			fmt.Printf("[%d/%d] Конвертация %s -> %s\n", idx+1, total, inputPath, outPath)
			err := convertPngToJpg(inputPath, outPath, quality, resize)
			if err != nil {
				fmt.Printf("  Ошибка при конвертации %s: %v\n", inputPath, err)
			}
		}(i, f)
	}
	wg.Wait()
	fmt.Println("Готово!")
}

func main() {
	quality := flag.Int("quality", 85, "Качество JPG (1-100)")
	resize := flag.String("resize", "", "Изменение размера (ШxВ)")
	output := flag.String("output", ".", "Папка для сохранения JPG")
	overwrite := flag.Bool("overwrite", false, "Перезаписывать существующие файлы")
	recursive := flag.Bool("recursive", false, "Рекурсивный обход папок")
	flag.Parse()
	inputs := flag.Args()
	if len(inputs) == 0 {
		fmt.Println("Использование: convert <PNG-файлы/папки> [--quality N] [--resize ШxВ] [--output DIR] [--overwrite] [--recursive]")
		return
	}
	processFiles(inputs, *quality, *resize, *output, *overwrite, *recursive)
}
