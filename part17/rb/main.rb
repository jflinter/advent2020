require 'pry'

class World
  attr_reader :points, :min_x, :max_x, :min_y, :max_y, :min_z, :max_z, :min_w, :max_w

  def initialize
    @min_x = 9999999
    @max_x = -9999999
    @min_y = 9999999
    @max_y = -9999999
    @min_z = 9999999
    @max_z = -9999999
    @min_w = 9999999
    @max_w = -9999999
    @points = Hash.new do |xs, k|
      xs[k] = Hash.new do |ys, k2|
        ys[k2] = Hash.new do |zs, k3|
          zs[k3] = {}
        end
      end
    end
  end

  def count
    @points.values.flat_map(&:values).flat_map(&:values).flat_map(&:values).count
  end

  def self.parse(input)
    world = World.new
    input.lines.each_with_index do |line, y|
      line.chars.each_with_index do |c, x|
        world.mark_occupied(x, y, 0, 0) if c == "#"
      end
    end
    world
  end

  def occupied?(x, y, z, w)
    @points[x][y][z][w]
  end

  def mark_occupied(x, y, z, w)
    @min_x = x if x < @min_x
    @max_x = x if x > @max_x
    @min_y = y if y < @min_y
    @max_y = y if y > @max_y
    @min_z = z if z < @min_z
    @max_z = z if z > @max_z
    @min_w = w if w < @min_w
    @max_w = w if w > @max_w
    @points[x][y][z][w] = 1
  end

  def neighbors(x, y, z, w)
    ((x-1)..(x+1)).to_a.product(
      ((y-1)..(y+1)).to_a,
      ((z-1)..(z+1)).to_a,
      ((w-1)..(w+1)).to_a,
    ) - [[x, y, z, w]]
  end

  def affected_spacepoints
    ((min_x-1)..(max_x+1)).to_a.product(
      ((min_y-1)..(max_y+1)).to_a,
      ((min_z-1)..(max_z+1)).to_a,
      ((min_w-1)..(max_w+1)).to_a,
    )
  end

  def occupied_neighbors(x, y, z, w)
    neighbors(x, y, z, w).count {|x, y, z, w| occupied?(x, y, z, w)}
  end

  def incr
    copy = World.new
    affected_spacepoints.each do |x, y, z, w|
      # If a cube is active and exactly 2 or 3 of its neighbors are also active, the cube remains active. Otherwise, the cube becomes inactive.
      # If a cube is inactive but exactly 3 of its neighbors are active, the cube becomes active. Otherwise, the cube remains inactive.
      occ = occupied_neighbors(x, y, z, w)
      if occupied?(x, y, z, w)
        copy.mark_occupied(x, y, z, w) if occ == 2 || occ == 3
      else
        copy.mark_occupied(x, y, z, w) if occ == 3
      end
    end
    copy
  end

  def debug2
    (-1..1).map do |z|
      "z=#{z}\n" + (-1..3).map do |y|
        (-1..3).map do |x|
          occupied_neighbors(x, y, z)
        end.join
      end.join("\n")
    end.join("\n\n")
  end

  def debug3
    (-1..1).map do |z|
      "z=#{z}\n" + (-1..3).map do |y|
        (-1..3).map do |x|
          occupied?(x, y, z) ? "#" : "."
        end.join
      end.join("\n")
    end.join("\n\n")
  end

  def debug
    (min_z..max_z).map do |z|
      "z=#{z}\n" + (min_y..max_y).map do |y|
        (min_x..max_x).map do |x|
          occupied?(x, y, z) ? "#" : "."
        end.join
      end.join("\n")
    end.join("\n\n")
  end
end

def main
  # input = ".#.\n..#\n###"
  input = File.read("input.txt")
  world = World.parse(input)
  copy = world
  6.times do
    copy = copy.incr
  end
  puts copy.count
end

main()
