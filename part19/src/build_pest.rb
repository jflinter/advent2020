#!/usr/bin/env ruby

require 'optparse'

def main
  options = {}
  OptionParser.new do |opt|
    opt.on('--input INPUT') { |o| options[:input] = o }
    opt.on('--output OUTPUT') { |o| options[:output] = o }
    opt.on('--munge') { |o| options[:munge] = true }
    opt.on('--sort') { |o| options[:sort] = true }
  end.parse!
  input = File.read(options[:input])
  rules = input.split("\n\n")[0]
  lines = rules.lines
  if options[:sort]
    lines = lines.sort_by do |line|
      line.split(": ")[0].to_i
    end
  end
  out = lines.map do |line|
    left, right = line.split(": ")
    i = left.strip.to_i
    if options[:munge]
      case i
      when 8 then right = "42 | 42 8"
      when 11 then right = "42 31 | 42 11 31"
      end
    end
    right = right.split(" | ").map do |r|
      "(" + r.split.join(" ~ ") + ")"
    end.join(" | ")
    base = i == 0 ? "#{left} = {SOI ~ (#{right}) ~ EOI}" : "#{left} = {#{right}}"
    base.gsub(/\d+/) {|v| "R#{v}"}
  end.join("\n")
  File.write(options[:output], out)
end

main()
