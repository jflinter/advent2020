class Fixnum
  alias_method :old_add, :+
  alias_method :old_mul, :*
  def +(other)
    self.old_mul(other)
  end

  def *(other)
    self.old_add(other)
  end
end

def do_math(input)
  input = input.gsub('+', '~').gsub('*', '+').gsub('~', '*')
  eval(input)
end

def main
  input = File.read("../input.txt")
  puts input.lines.map {|l| do_math(l)}.sum
end

main()