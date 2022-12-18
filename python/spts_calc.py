#   Python cause Quack
#   Made by iamSkev#4260

from __future__ import annotations

import re
from datetime import timedelta

STATS = {"fs": 1, "bt": 1.25, "psy": 1.5, 'idk': "idk"}

def _converter(stat) -> int | None:    
    suffix_converter = {
        "k": 1_000,
        "m": 1_000_000,
        "b": 1_000_000_000,
        "t": 1_000_000_000_000,
        "qa": 1_000_000_000_000_000,
        "qi": 1_000_000_000_000_000_000
    }
    try:
        if re.search("[a-zA-Z]+", stat):
            digit = float((re.sub("[a-zA-Z]+", "", stat)))
            letter = (re.sub("[^a-zA-Z]", "", stat)).lower()
            return (
                float(digit * (suffix_converter.get(letter)))
                if suffix_converter.get(letter) is not None
                else None
            )
    except ValueError:
        return None
    return float(stat)


def _net(message=None, suffix=True) -> int | None:
    while True:
        if suffix is True:
            print(
                "i'm sorry but you'd have to type that out again since that doesn't look right."
            )
            print(
                "You can either add a suffix to your stat or type it out manually. e.g., 69m or 69000000 "
            )
            print(
            """ Here are the suffixes:

            K (Thousand)
            M (Million)
            B (Billion)
            T (Trillion)
            Qa (Quadrillion)
            Qi (Quintillion)
            """
            )
            stat = _converter((input(message)))
            if stat is not None:
                return stat


        else:
            print(
            """ I'm sorry but you'll have to specify which stat you're training between: 
                
            fs (Fist Strength)
            bt (Body Toughness) 
            psy (Psychic Power)
            idk (i don't know)
            """ 
            )
            stat = str(input("What stat are you training? (fs, bt, psy, idk): ")).lower()
            if STATS.get(stat) is not None:
                return stat

def _time_checker(time, msg) -> 0:
    return f"{time} {msg+'s' if time > 1 else msg} " if time != 0 else ""

def _get_time(goal, current, per_tick, tick_time) -> str:
    optimal_time = ((((goal - current) / per_tick) * (STATS.get(tick_time))) / 60) / 60
    
    time = timedelta(hours=optimal_time)
    
    days, hours, minutes = time.days, time.seconds // 3600, (time.seconds % 3600) // 60
    
    seconds = time.seconds - ((hours*3600)+(minutes*60))
    
    years, months = days // 365, days // 30
    
    days, months = days - (months*30), months - (years*12)
    
    milliseconds, microseconds = int((time.microseconds/10000)*10), ((((time.microseconds/10000)*10)-(int((time.microseconds/10000)*10)))*1000)
    
    return f"{_time_checker(years, 'year')}{'and ' if months == 0 and years != 0 else ''}{_time_checker(months, 'month')}{'and ' if days == 0 and months != 0 else ''}{_time_checker(days, 'day')}{'and ' if minutes == 0 and hours != 0 else ''}{_time_checker(hours, 'hour')}{'and ' if seconds == 0 and minutes != 0 else ''}{_time_checker(minutes, 'minute')}{'and ' if milliseconds == 0 and seconds != 0 else ''}{_time_checker(seconds, 'second')}{'and ' if microseconds == 0 and milliseconds != 0 else ''}{_time_checker(milliseconds, 'millisecond')}{'and' if microseconds != 0 else ''} {_time_checker(microseconds, 'microsecond')}"

while True:
    print("You can add suffixes to your stats e.g., 69m will be the same as 69000000")
    goal = _converter((input("How much is your goal?: ")))
    if goal is None:
        goal = _net("How much is your goal?: ")
    current = _converter(input("How much do you currently have?: "))
    if current is None:
        current = _net("How much do you currently have?: ")

    per_tick = _converter(input("How much do you get per tick?: "))
    if per_tick is None:
        per_tick = _net("How much do you get per tick?: ")

    tick_time = str(input("What stat are you training? (fs, bt, psy, idk): ")).lower()
    if tick_time not in ('fs', 'bt', 'psy', "idk"):
        tick_time = _net(suffix=False)
    
    if tick_time == "idk":
        total_time = []
        
        for k, v in STATS.items():
            if k == "idk":
                continue
            time_result = _get_time(goal, current, per_tick, k)
            total_time.append(time_result)
        
        print(
        f"""
Since you picked idk (i don't know) when i asked you about what you're training, i calculated the time for all 3 of the stats.

Here they are:

Fist Strength: {total_time[0]}
Body Toughness: {total_time[1]}
Psychic Power: {total_time[2]}""")
        print("""
Good Luck. And as always. Quack.
        """
        )
    
    else:
    
        time_result = _get_time(goal, current, per_tick, tick_time)
        print(
            f"It would take roughly around {time_result}"
        )
        print("Good Luck. And as always. Quack.")
