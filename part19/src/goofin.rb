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
  map = {}
  lines.each do |line|
    left, right = line.split(": ")
    if right.include?("a") || right.include?("b")
      map[left] = right.gsub("\"", "").strip
    elsif right.include?("|")
      tr = right.split(' | ').map do |p|
        inner = p.split.map {|q| "(#{q})"}.join
        "(#{inner})"
      end.join('|')
      tr = "(#{tr})"
      map[left] = tr
    else
      tr = right.split.map {|q| "(#{q})"}.join
      tr = "(#{tr})" if right.split.count > 1
      map[left] = tr
    end
  end
  if options[:munge]
    map['8'] = "((42)+)"
    map['11'] = '(' + (1..10).map {|i| "((42){#{i}}(31){#{i}})"}.join("|") + ')'
  end
  str = "^(0)$"
  rex = /\((\d+)\)/
  i = 0
  while (match = rex.match(str))
    val = match[0] # "0"
    rep = map[match[1]]
    # puts "hello: #{str}, val: #{val}, rep: #{rep}" if i < 6
    str = str.gsub(val, rep)
    i += 1
  end
  regex = Regexp.new(str)
  puts regex
  tests = input.split("\n\n")[1]
  puts tests.lines
  puts "matching: #{tests.lines.count {|t| regex.match(t)}}"
end

main()
