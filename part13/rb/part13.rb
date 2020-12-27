def chinese(ms, as)
  big_m = ms.inject(:*)
  zs = ms.map {|m| big_m / m}
  ys = zs.zip(ms).map {|z, m| invmod(z, m)}
  ws = ys.zip(zs).map {|y, z| (y*z) % big_m}
  as.zip(ws).map {|a, w| a * w}.sum % big_m
end

def extended_gcd(a, b)
  last_remainder, remainder = a.abs, b.abs
  x, last_x, y, last_y = 0, 1, 1, 0
  while remainder != 0
    last_remainder, (quotient, remainder) = remainder, last_remainder.divmod(remainder)
    x, last_x = last_x - quotient*x, x
    y, last_y = last_y - quotient*y, y
  end
  return last_remainder, last_x * (a < 0 ? -1 : 1)
end
 
def invmod(e, et)
  g, x = extended_gcd(e, et)
  if g != 1
    raise 'Multiplicative inverse modulo does not exist!'
  end
  x % et
end

x = File.readlines('../input.txt')[1]
# input = "7,13,x,x,59,x,31,19"
input = x.split(",").each_with_index.filter_map {|m, a| [m.to_i, m.to_i - a] if m.to_i != 0}
ms = input.map {|i| i[0]}
as = input.map {|i| i[1]}
puts "#{ms}, #{as}"
puts chinese(ms, as)
