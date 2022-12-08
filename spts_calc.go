/*
	Go cause yes.
	Made by iamSkev#4260
*/

package main

import (
	"bufio"
	"fmt"
	"math/big"
	"os"
	"regexp"
	"strings"
	"golang.org/x/exp/slices"
)

var SUFFIX_REGEX *regexp.Regexp = regexp.MustCompile("[a-zA-Z]+")
var NUMBER_REGEX *regexp.Regexp = regexp.MustCompile("[^a-zA-Z]")

var TICK_TIMES map[string]*big.Float = map[string]*big.Float{
	"fs":  big.NewFloat(1),
	"bt":  big.NewFloat(1.25),
	"psy": big.NewFloat(1.5),
	"idk": big.NewFloat(69),
}


var SUFFIX_CONVERTER map[string]*big.Float = map[string]*big.Float{
	"k":  big.NewFloat(1_000),
	"m":  big.NewFloat(1_000_000),
	"b":  big.NewFloat(1_000_000_000),
	"t":  big.NewFloat(1_000_000_000_000),
	"qa": big.NewFloat(1_000_000_000_000_000),
	"qi": big.NewFloat(1_000_000_000_000_000_000),
}

var TICK_OPTIONS []string = []string{"bt", "fs", "psy", "idk"}


func net(message string, suffix bool) (*big.Float, string) {
	scanner := bufio.NewScanner(os.Stdin)

	for {
		if suffix {
			fmt.Println("i'm sorry but you'd have to type that out again since that doesn't look right.")
			fmt.Println("You can either add a suffix to your stat or type it out manually. e.g., 69m or 69000000")
			fmt.Println(`Here are the suffixes:

	K (Thousand)
	M (Million)
	B (Billion)
	T (Trillion)
	Qa (Quadrillion)
	Qi (Quintillion)
	`)
			fmt.Print(message)
			scanner.Scan()
			val := converter(scanner.Text())
			fmt.Println(val, scanner.Text())
			if val != nil {
				return val, ""
			}
		} else {
			fmt.Println(`I'm sorry but you'll have to specify which stat you're training between: 
                
            fs (Fist Strength)
            bt (Body Toughness) 
            psy (Psychic Power)
            idk (i don't know)`)
			fmt.Print(message)
			scanner.Scan()
			val := slices.Contains(TICK_OPTIONS, scanner.Text())
			if val {
				return big.NewFloat(0), scanner.Text()
			}
		}
	}
}

func converter(stat string) *big.Float {
	var number string
	var true_stat *big.Float
	var suffix string
	number = stat
	
	if SUFFIX_REGEX.MatchString(stat) {
		number = strings.ToLower(SUFFIX_REGEX.ReplaceAllString(stat, ""))
		suffix = strings.ToLower(NUMBER_REGEX.ReplaceAllString(stat, ""))
		if SUFFIX_CONVERTER[suffix] == nil {
			return nil
		} else if  SUFFIX_CONVERTER[suffix] != nil && number == "" {
			return nil
		}
	}
	arr, ok := (big.NewFloat(0)).SetString(number)
	if !ok {
		return nil
	} else if ok && suffix != "" {
		var suffix_amount *big.Float = SUFFIX_CONVERTER[suffix]
		if suffix_amount == nil {
			return nil
		} else {
			true_stat = new(big.Float).Mul(arr, suffix_amount)
		}
	} else if ok && suffix == "" {
		true_stat = arr
	}

	return true_stat
}

func correct_grammar(time_amount *big.Float, time_name string) string {
	var correct_string string

	if time_amount.Cmp(big.NewFloat(1)) == 1 {
		correct_string = fmt.Sprintf("%g %s", time_amount, time_name+"s")
	} else {
		correct_string = fmt.Sprintf("%g %s", time_amount, time_name)
	}

	return correct_string
}

func calculate(apt, amt_to_get, current_amt *big.Float, tick string) string {
	var amt_of_time []string
	var time_result string

	var tick_amount *big.Float = TICK_TIMES[tick]

	amt_to_get = new(big.Float).Sub(amt_to_get, current_amt)

	var stats *big.Float = new(big.Float).Quo(new(big.Float).Quo((new(big.Float).Mul((new(big.Float).Quo(amt_to_get, apt)), tick_amount)), big.NewFloat(60)), big.NewFloat(60))
	var rounded_stats *big.Float = big.NewFloat(0)
	var throwaway *big.Int
	var _ big.Accuracy
	throwaway, _ = stats.Int(nil)
	rounded_stats.SetInt(throwaway)

	var hours *big.Float = big.NewFloat(0)

	var seconds *big.Float = big.NewFloat(0)
	var minutes *big.Float = big.NewFloat(0)
	var milliseconds *big.Float = big.NewFloat(0)
	var microseconds *big.Float = big.NewFloat(0)
	var nanoseconds *big.Float = big.NewFloat(0)

	var days *big.Float

	if rounded_stats.Cmp(big.NewFloat(24)) == 1 || rounded_stats.Cmp(big.NewFloat(24)) == 0 {
		days = new(big.Float).Quo(rounded_stats, big.NewFloat(24))

		throwaway, _ = days.Int(nil)

		days.SetInt(throwaway)

		hours = new(big.Float).Sub(rounded_stats, (new(big.Float).Mul(days, big.NewFloat(24))))

		if days.Cmp(big.NewFloat(1)) == 1 || days.Cmp(big.NewFloat(1)) == 0 {
			amt_of_time = append(amt_of_time, correct_grammar(days, "day"))
		}

		if hours.Cmp(big.NewFloat(1)) == 1 || hours.Cmp(big.NewFloat(1)) == 0 {
			amt_of_time = append(amt_of_time, correct_grammar(hours, "hour"))
		}
	} else if rounded_stats.Cmp(big.NewFloat(0)) == 1 {
		hours = rounded_stats
		amt_of_time = append(amt_of_time, correct_grammar(hours, "hour"))

	}

	var rounded_minutes *big.Float = big.NewFloat(0)
	var rounded_seconds *big.Float = big.NewFloat(0)
	var rounded_milliseconds *big.Float = big.NewFloat(0)
	var rounded_microseconds *big.Float = big.NewFloat(0)
	var rounded_nanoseconds *big.Float = big.NewFloat(0)

	new(big.Float).Sub(stats, rounded_stats)
	minutes = new(big.Float).Mul((new(big.Float).Sub(stats, rounded_stats)), big.NewFloat(60))
	throwaway, _ = minutes.Int(nil)
	rounded_minutes.SetInt(throwaway)

	seconds = new(big.Float).Mul((new(big.Float).Sub(minutes, rounded_minutes)), big.NewFloat(60))
	throwaway, _ = seconds.Int(nil)
	rounded_seconds.SetInt(throwaway)

	milliseconds = new(big.Float).Mul((new(big.Float).Sub(seconds, rounded_seconds)), big.NewFloat(1000))
	throwaway, _ = milliseconds.Int(nil)
	rounded_milliseconds.SetInt(throwaway)

	microseconds = new(big.Float).Mul((new(big.Float).Sub(milliseconds, rounded_milliseconds)), big.NewFloat(1000))
	throwaway, _ = microseconds.Int(nil)
	rounded_microseconds.SetInt(throwaway)

	nanoseconds = new(big.Float).Mul((new(big.Float).Sub(microseconds, rounded_microseconds)), big.NewFloat(1000))
	throwaway, _ = nanoseconds.Int(nil)
	rounded_nanoseconds.SetInt(throwaway)

	if rounded_minutes.Cmp(big.NewFloat(0)) == 1 {
		amt_of_time = append(amt_of_time, correct_grammar(rounded_minutes, "minute"))
	}
	if rounded_seconds.Cmp(big.NewFloat(0)) == 1 {
		amt_of_time = append(amt_of_time, correct_grammar(rounded_seconds, "second"))
	}
	if rounded_milliseconds.Cmp(big.NewFloat(0)) == 1 {
		amt_of_time = append(amt_of_time, correct_grammar(rounded_milliseconds, "millisecond"))
	}
	if rounded_microseconds.Cmp(big.NewFloat(0)) == 1 {
		amt_of_time = append(amt_of_time, correct_grammar(rounded_microseconds, "microsecond"))
	}
	if rounded_nanoseconds.Cmp(big.NewFloat(0)) == 1 {
		amt_of_time = append(amt_of_time, correct_grammar(rounded_nanoseconds, "nanosecond"))
	}

	if len(amt_of_time) > 1 {
		time_result = strings.Join(amt_of_time[:len(amt_of_time)-1], ", ") + fmt.Sprintf(" and %s.", amt_of_time[len(amt_of_time)-1])
	} else if len(amt_of_time) == 1 {
		// make time_result the first element if the list is as short my alive time whenever a random fucking clan member decides to kill the soft fluffy bean bag that's taking 60% of the space in the bt area
		time_result = amt_of_time[0]
	} else {
		time_result = "now"
	}
	
	return time_result
}

func ask_questions() string {
	var total_time string = `
Since you picked idk (i don't know) when i asked you about what you're training, i calculated the time for all 3 of the stats.
	
Here they are:
	`
	scanner := bufio.NewScanner(os.Stdin)
	fmt.Print("How much is your goal?: ")
	scanner.Scan()
	var goal *big.Float = converter(scanner.Text())
	if goal == nil {
		goal, _ = net("How much is your goal?: ", true)
	}
	fmt.Print("\nHow much do you currently have?: ")
	scanner.Scan()
	var current *big.Float = converter(scanner.Text())
	if current == nil {
		current, _ = net("How much do you currently have?: ", true)
	}
	fmt.Print("\nHow much do you get per tick?: ")
	scanner.Scan()
	var per_tick *big.Float = converter(scanner.Text())
	if per_tick == nil {
		per_tick, _ = net("How much do you get per tick?: ", true)
	}
	fmt.Print("\nWhat stat are you training?(fs, bt, psy, idk): ")
	scanner.Scan()
	var tick_time string = scanner.Text()
	if !slices.Contains(TICK_OPTIONS, scanner.Text()) {
		_, tick_time = net("What stat are you training?(fs, bt, psy, idk): ", false)
	}
	if tick_time == "idk" {
		for _, tick := range TICK_OPTIONS {
			
			if tick == "bt" {
				total_time += fmt.Sprintf("\nBody Toughness: it would take roughly around %s", calculate(per_tick, goal, current, tick))
			} else if tick == "fs" {
				total_time += fmt.Sprintf("\nFist Strenght: it would take roughly around %s", calculate(per_tick, goal, current, tick))
			} else if tick == "psy" {
				total_time += fmt.Sprintf("\nPsychic Power: it would take roughly around %s\n", calculate(per_tick, goal, current, tick))
			} else {
				break
			}
			}
		return total_time
		}
	value := calculate(per_tick, goal, current, tick_time)
	return fmt.Sprintf("\nIt would take rougly around %s\n", value)
}

func main() {
	for {
		fmt.Println(ask_questions())
		fmt.Println("Good Luck. And as always. Quack.")
	}
}
