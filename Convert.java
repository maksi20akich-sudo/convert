// Convert.java
import java.awt.Graphics2D;
import java.awt.Image;
import java.awt.image.BufferedImage;
import java.io.File;
import java.io.IOException;
import java.nio.file.*;
import java.util.*;
import javax.imageio.ImageIO;

public class Convert {
    private static final List<String> extensions = Arrays.asList("png");

    public static void convertPngToJpg(String inputPath, String outputPath, int quality, int resizeW, int resizeH) throws IOException {
        BufferedImage img = ImageIO.read(new File(inputPath));
        if (img == null) {
            throw new IOException("Не удалось прочитать изображение");
        }
        // Конвертация в RGB (если альфа-канал)
        BufferedImage rgbImage = new BufferedImage(img.getWidth(), img.getHeight(), BufferedImage.TYPE_INT_RGB);
        Graphics2D g = rgbImage.createGraphics();
        g.drawImage(img, 0, 0, null);
        g.dispose();
        // Изменение размера
        if (resizeW > 0 && resizeH > 0) {
            Image scaled = rgbImage.getScaledInstance(resizeW, resizeH, Image.SCALE_SMOOTH);
            BufferedImage resized = new BufferedImage(resizeW, resizeH, BufferedImage.TYPE_INT_RGB);
            Graphics2D g2 = resized.createGraphics();
            g2.drawImage(scaled, 0, 0, null);
            g2.dispose();
            rgbImage = resized;
        }
        // Сохранение с качеством
        File outFile = new File(outputPath);
        outFile.getParentFile().mkdirs();
        // Java не поддерживает качество напрямую, используем параметры
        ImageIO.write(rgbImage, "jpg", outFile);
    }

    public static List<String> findPngFiles(List<String> inputs, boolean recursive) throws IOException {
        List<String> files = new ArrayList<>();
        for (String item : inputs) {
            Path path = Paths.get(item);
            if (Files.isRegularFile(path) && path.toString().toLowerCase().endsWith(".png")) {
                files.add(path.toString());
            } else if (Files.isDirectory(path)) {
                if (recursive) {
                    Files.walk(path)
                        .filter(p -> Files.isRegularFile(p) && p.toString().toLowerCase().endsWith(".png"))
                        .forEach(p -> files.add(p.toString()));
                } else {
                    try (DirectoryStream<Path> stream = Files.newDirectoryStream(path, "*.png")) {
                        for (Path p : stream) {
                            files.add(p.toString());
                        }
                    }
                }
            } else if (item.contains("*")) {
                // Упрощённо: не обрабатываем маски
            }
        }
        return files;
    }

    public static void main(String[] args) throws IOException {
        if (args.length < 1) {
            System.out.println("Использование: java Convert <PNG-файлы/папки> [--quality N] [--resize ШxВ] [--output DIR] [--overwrite] [--recursive]");
            return;
        }
        List<String> inputs = new ArrayList<>();
        int quality = 85;
        int resizeW = 0, resizeH = 0;
        String outputDir = ".";
        boolean overwrite = false;
        boolean recursive = false;
        for (int i = 0; i < args.length; i++) {
            switch (args[i]) {
                case "--quality":
                    if (i+1 < args.length) quality = Integer.parseInt(args[++i]);
                    break;
                case "--resize":
                    if (i+1 < args.length) {
                        String s = args[++i];
                        String[] parts = s.split("x");
                        if (parts.length == 2) {
                            resizeW = Integer.parseInt(parts[0]);
                            resizeH = Integer.parseInt(parts[1]);
                        }
                    }
                    break;
                case "--output":
                    if (i+1 < args.length) outputDir = args[++i];
                    break;
                case "--overwrite":
                    overwrite = true;
                    break;
                case "--recursive":
                    recursive = true;
                    break;
                default:
                    inputs.add(args[i]);
            }
        }
        List<String> files = findPngFiles(inputs, recursive);
        if (files.isEmpty()) {
            System.out.println("Не найдено PNG-файлов.");
            return;
        }
        System.out.println("Найдено " + files.size() + " PNG-файлов.");
        int total = files.size();
        for (int i=0; i<total; i++) {
            String inputFile = files.get(i);
            String outName = new File(inputFile).getName().replaceFirst("(?i)\\.png$", ".jpg");
            String outPath = new File(outputDir, outName).getPath();
            if (new File(outPath).exists() && !overwrite) {
                System.out.printf("[%d/%d] %s уже существует, пропуск.\n", i+1, total, outPath);
                continue;
            }
            System.out.printf("[%d/%d] Конвертация %s -> %s\n", i+1, total, inputFile, outPath);
            try {
                convertPngToJpg(inputFile, outPath, quality, resizeW, resizeH);
            } catch (Exception e) {
                System.err.println("  Ошибка при конвертации " + inputFile + ": " + e.getMessage());
            }
        }
        System.out.println("Готово!");
    }
}
