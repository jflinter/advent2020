require 'set'

# input = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)\ntrh fvjkl sbzzf mxmxvkd (contains dairy)\nsqjhc fvjkl (contains soy)\nsqjhc mxmxvkd sbzzf (contains fish)"
input = File.read('input.txt')

parsed = input.lines.map do |line|
  left, right = line.strip.split(' (contains ')
  ingredients = Set.new(left.split)
  allergens = Set.new(right[0...-1].split(', '))
  [ingredients, allergens]
end

all_ingredients = Set.new
all_allergens = Set.new
possibilities = {}

parsed.each do |ingredients, allergens|
  all_ingredients += ingredients
  all_allergens += allergens

  allergens.each do |allergen|
    possibilities[allergen] ||= ingredients
    possibilities[allergen] = possibilities[allergen] & ingredients 
  end
end

confirmed = {}
while possibilities.any? do
  known = possibilities.find {|_, v| v.count == 1}
  allergen = known[0]
  ingredient = known[1].first
  confirmed[allergen] = ingredient
  possibilities.delete(allergen)
  possibilities = Hash[possibilities.map do |k, v|
    [k, v - [ingredient]]
  end]
end

puts confirmed.keys.sort.map {|k| confirmed[k]}.join(',')
