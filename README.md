# Sup

Sup is meant to keep us up to date with what's going on in the tech world. It's written in Rust because Rust is trendy as heck.

Right now it only fetches the top X stories from hackernews and their URLs. The HN data is sourced from their [public API](https://github.com/HackerNews/API)

Build:
```
cargo build
```

Run:
```
cargo run
```

Example run:
```
💀 sup git:(main) cargo run                                                               
   Compiling sup v0.1.0 (/home/tony/code/smithingsoftware/sup)
    Finished dev [unoptimized + debuginfo] target(s) in 2.01s
     Running `target/debug/sup`
Getting top 10 stories from HackerNews...
Monarch: Google’s Planet-Scale In-Memory Time Series Database https://www.micahlerner.com/2022/04/24/monarch-googles-planet-scale-in-memory-time-series-database.html 1652544731 125 29
You can create a great looking websites while sucking at design https://thefullstackdev.net/resource/create-beautiful-website-while-sucking-at-design/ 1652545472 114 47
“Blown Away Guy”: Iconic music image https://www.vintag.es/2022/02/blown-away-guy.html 1652546269 63 27
Hundreds of patient data breaches are left unpunished https://www.bmj.com/content/377/bmj.o1126 1652553878 24 1
‎Cracking the Code: Sneakers at 30 https://letterboxd.com/journal/cracking-the-code-sneakers/ 1652539417 154 33
Bad government policy is fueling the infant formula shortage https://reason.com/volokh/2022/05/13/how-bad-government-policy-is-fueling-the-infant-formula-shortage/ 1652544733 104 114
Eurovision Song Contest Bingo https://eurovision.vonmalmborg.com/ 1652555530 24 10
Time-Series Compression Algorithms https://www.timescale.com/blog/time-series-compression-algorithms-explained/ 1652544496 73 8
Linux Insides https://0xax.gitbooks.io/linux-insides/content/ 1652536230 146 9
The Antikythera Mechanism Episode 11 – Inscribing the Back Plate – Part 1 https://www.youtube.com/watch?v=iBRCL090PxA 1652543565 27 3
```