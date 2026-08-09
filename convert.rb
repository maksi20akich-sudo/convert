# convert.rb
require 'mini_magick'
require 'find'
require 'optparse'

def convert_png_to_jpg(input_path, output_path, quality, resize_w, resize_h)
  image = MiniMagick::Image.open(input_path)
  # Конвертация в JPG (автоматически удаляет альфа-канал)
  image.format 'jpg'
  image.quality quality
  if resize_w > 0 && resize_h > 0
    image.resize "#{resize_w}x#{resize_h}"
  end
  image.write output_path
end

def process_files(inputs, quality, resize_w, resize_h, output_dir, overwrite, recursive)
  files = []
  inputs.each do |item|
    if File.file?(item) && item.downcase.end_with?('.png')
      files << item
    elsif File.directory?(item)
      if recursive
        Find.find(item) do |path|
          files << path if File.file?(path) && path.downcase.end_with?('.png')
        end
      else
        Dir.glob(File.join(item, '*.png')).each { |f| files << f }
      end
    elsif item.include?('*')
      Dir.glob(item).each { |f| files << f if f.downcase.end_with?('.png') }
    end
  end
  if files.empty?
    puts "Не найдено PNG-файлов."
    return
  end
  Dir.mkdir(output_dir) unless Dir.exist?(output_dir)
  total = files.size
  puts "Найдено #{total} PNG-файлов."
  files.each_with_index do |input_file, idx|
    out_name = File.basename(input_file, '.png') + '.jpg'
    out_path = File.join(output_dir, out_name)
    if File.exist?(out_path) && !overwrite
      puts "[#{idx+1}/#{total}] #{out_path} уже существует, пропуск."
      next
    end
    puts "[#{idx+1}/#{total}] Конвертация #{input_file} -> #{out_path}"
    begin
      convert_png_to_jpg(input_file, out_path, quality, resize_w, resize_h)
    rescue => e
      puts "  Ошибка при конвертации #{input_file}: #{e.message}"
    end
  end
  puts "Готово!"
end

options = {}
OptionParser.new do |opts|
  opts.banner = "Использование: ruby convert.rb <PNG-файлы/папки> [опции]"
  opts.on("--quality N", Integer, "Качество JPG (1-100)") { |v| options[:quality] = v }
  opts.on("--resize ШxВ", String, "Изменение размера") { |v| options[:resize] = v }
  opts.on("--output DIR", String, "Папка для сохранения") { |v| options[:output] = v }
  opts.on("--overwrite", "Перезаписывать") { options[:overwrite] = true }
  opts.on("--recursive", "Рекурсивный обход") { options[:recursive] = true }
end.parse!

quality = options[:quality] || 85
resize = options[:resize] ? options[:resize].split('x').map(&:to_i) : [0,0]
output_dir = options[:output] || '.'
overwrite = options[:overwrite] || false
recursive = options[:recursive] || false
inputs = ARGV

if inputs.empty?
  puts "Не указаны файлы или папки."
  exit
end

process_files(inputs, quality, resize[0], resize[1], output_dir, overwrite, recursive)
