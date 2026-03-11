// ============================================================================
// Shared & Reusable Constants
// ============================================================================

export const ANY = "Any";

export const GENRES = [
	"Action", "Adventure", "Comedy", "Drama", "Ecchi", "Fantasy", "Horror", "Mahou Shoujo",
	"Mecha", "Music", "Mystery", "Psychological", "Romance", "Sci-Fi", "Slice Of Life", 
	"Sports", "Supernatural", "Thriller", "Hentai"
];

export const DEMOGRAPHICS = [
	"Josei", "Kids", "Seinen", "Shoujo", "Shounen"
];

export const ADVANCED_TAGS = {
	"Cast / Main Cast": [
		"Anti-Hero", "Elderly Protagonist", "Ensemble Cast", "Estranged Family", 
		"Female Protagonist", "Male Protagonist", "Primarily Adult Cast", 
		"Primarily Animal Cast", "Primarily Child Cast", "Primarily Female Cast", 
		"Primarily Male Cast", "Primarily Teen Cast"
	],
	"Cast / Traits": [
		"Age Regression", "Agender", "Aliens", "Amnesia", "Angels", "Anthropomorphism", 
		"Aromantic", "Arranged Marriage", "Artificial Intelligence", "Asexual", "Bisexual", 
		"Butler", "Centaur", "Chimera", "Chuunibyou", "Clone", "Cosplay", "Cowboys", 
		"Crossdressing", "Cyborg", "Delinquents", "Demons", "Detective", "Dinosaurs", 
		"Disability", "Dissociative Identities", "Dragons", "Dullahan", "Elf", "Fairy", 
		"Femboy", "Ghost", "Goblin", "Gods", "Gyaru", "Hikikomori", "Homeless", "Idol", 
		"Kemonomimi", "Kuudere", "Maids", "Mermaid", "Monster Boy", "Monster Girl", 
		"Nekomimi", "Ninja", "Nudity", "Nun", "Office Lady", "Oiran", "Ojou-Sama", "Orphan", 
		"Pirates", "Robots", "Samurai", "Shrine Maiden", "Skeleton", "Succubus", "Tanned Skin", 
		"Teacher", "Tomboy", "Transgender", "Tsundere", "Twins", "Vampire", "Veterinarian", 
		"Vikings", "Villainess", "VTuber", "Werewolf", "Witch", "Yandere", "Zombie"
	],
	"Demographic": DEMOGRAPHICS,
	"Setting": ["Matriarchy"],
	"Setting / Scene": [
		"Bar", "Boarding School", "Camping", "Circus", "Coastal", "College", "Desert", 
		"Dungeon", "Foreign", "Inn", "Konbini", "Natural Disaster", "Office", "Outdoor Activities", 
		"Prison", "Restaurant", "Rural", "School", "School Club", "Snowscape", "Urban", 
		"Wilderness", "Work"
	],
	"Setting / Time": [
		"Achronological Order", "Anachronism", "Ancient China", "Dystopian", "Historical", 
		"Medieval", "Time Skip"
	],
	"Setting / Universe": [
		"Afterlife", "Alternate Universe", "Augmented Reality", "Omegaverse", "Post-Apocalyptic", 
		"Space", "Urban Fantasy", "Virtual World"
	],
	"Technical": [
		"4-Koma", "Achromatic", "Advertisement", "Anthology", "CGI", "Episodic", "Flash", 
		"Full CGI", "Full Color", "Long Strip", "Mixed Media", "No Dialogue", "Non-Fiction", 
		"POV", "Puppetry", "Rotoscoping", "Stop Motion", "Vertical Video"
	],
	"Theme / Action": [
		"Archery", "Battle Royale", "Espionage", "Fugitive", "Guns", "Martial Arts", 
		"Spearplay", "Swordplay"
	],
	"Theme / Arts": [
		"Acting", "Ballet", "Calligraphy", "Classic Literature", "Drawing", "Fashion", "Food", 
		"Kabuki", "Makeup", "Manzai", "Modeling", "Photography", "Rakugo", "Writing"
	],
	"Theme / Arts-Music": [
		"Band", "Classical Music", "Dancing", "Hip-Hop Music", "Jazz Music", "Metal Music", 
		"Musical Theater", "Rock Music"
	],
	"Theme / Comedy": [
		"Parody", "Satire", "Slapstick", "Surreal Comedy"
	],
	"Theme / Drama": [
		"Bullying", "Class Struggle", "Coming Of Age", "Conspiracy", "Eco-Horror", 
		"Fake Relationship", "Kingdom Management", "Rehabilitation", "Revenge", "Suicide", 
		"Tragedy"
	],
	"Theme / Fantasy": [
		"Alchemy", "Body Swapping", "Cultivation", "Curses", "Exorcism", "Fairy Tale", 
		"Henshin", "Isekai", "Kaiju", "Magic", "Mythology", "Necromancy", "Reverse Isekai", 
		"Shapeshifting", "Steampunk", "Super Power", "Superhero", "Wuxia", "Youkai"
	],
	"Theme / Game": ["Board Game", "E-Sports", "Video Games"],
	"Theme / Game-Card & Board Game": ["Card Battle", "Go", "Karuta", "Mahjong", "Poker", "Shogi"],
	"Theme / Game-Sport": [
		"Acrobatics", "Airsoft", "American Football", "Athletics", "Badminton", "Baseball", 
		"Basketball", "Bowling", "Boxing", "Cheerleading", "Cycling", "Fencing", "Fishing", 
		"Fitness", "Football", "Golf", "Handball", "Ice Skating", "Judo", "Lacrosse", "Parkour", 
		"Rugby", "Scuba Diving", "Skateboarding", "Sumo", "Surfing", "Swimming", "Table Tennis", 
		"Tennis", "Volleyball", "Wrestling"
	],
	"Theme / Other": [
		"Adoption", "Animals", "Astronomy", "Autobiographical", "Biographical", "Blackmail", 
		"Body Horror", "Body Image", "Cannibalism", "Chibi", "Cosmic Horror", "Creature Taming", 
		"Crime", "Crossover", "Death Game", "Denpa", "Drugs", "Economics", "Educational", 
		"Environmental", "Ero Guro", "Filmmaking", "Found Family", "Gambling", "Gender Bending", 
		"Gore", "Indigenous Cultures", "Language Barrier", "LGBTQ+ Themes", "Lost Civilization", 
		"Marriage", "Medicine", "Memory Manipulation", "Meta", "Mountaineering", "Noir", 
		"Otaku Culture", "Pandemic", "Philosophy", "Politics", "Pregnancy", "Proxy Battle", 
		"Psychosexual", "Reincarnation", "Religion", "Rescue", "Royal Affairs", "Slavery", 
		"Software Development", "Survival", "Terrorism", "Torture", "Travel", "Vocal Synth", "War"
	],
	"Theme / Other-Organisations": [
		"Assassins", "Criminal Organization", "Cult", "Firefighters", "Gangs", "Mafia", 
		"Military", "Police", "Triads", "Yakuza"
	],
	"Theme / Other-Vehicle": [
		"Aviation", "Cars", "Mopeds", "Motorcycles", "Ships", "Tanks", "Trains"
	],
	"Theme / Romance": [
		"Age Gap", "Boys' Love", "Cohabitation", "Female Harem", "Heterosexual", "Love Triangle", 
		"Male Harem", "Matchmaking", "Mixed Gender Harem", "Polyamorous", "Teens' Love", 
		"Unrequited Love", "Yuri"
	],
	"Theme / Sci Fi": [
		"Cyberpunk", "Space Opera", "Time Loop", "Time Manipulation", "Tokusatsu"
	],
	"Theme / Sci Fi-Mecha": ["Real Robot", "Super Robot"],
	"Theme / Slice Of Life": [
		"Agriculture", "Cute Boys Doing Cute Things", "Cute Girls Doing Cute Things", 
		"Family Life", "Horticulture", "Iyashikei", "Parenthood"
	],
	"Sexual Content": [
		"Ahegao", "Amputation", "Anal Sex", "Armpits", "Ashikoki", "Asphyxiation", "Bondage", 
		"Boobjob", "Cervix Penetration", "Cheating", "Cumflation", "Cunnilingus", "Deepthroat", 
		"Defloration", "DILF", "Double Penetration", "Erotic Piercings", "Exhibitionism", "Facial", 
		"Feet", "Fellatio", "Femdom", "Fingering", "Fisting", "Flat Chest", "Futanari", "Group Sex", 
		"Hair Pulling", "Handjob", "Human Pet", "Hypersexuality", "Incest", "Inseki", "Irrumatio", 
		"Lactation", "Large Breasts", "Male Pregnancy", "Masochism", "Masturbation", "Mating Press", 
		"MILF", "Nakadashi", "Netorare", "Netorase", "Netori", "Oyakodon", "Pet Play", "Prostitution", 
		"Public Sex", "Rape", "Rimjob", "Sadism", "Scat", "Scissoring", "Sex Toys", "Shimaidon", 
		"Squirting", "Sumata", "Swapping", "Sweat", "Tentacles", "Threesome", "Virginity", "Vore", 
		"Voyeur", "Watersports", "Zoophilia"
	]
};

// ============================================================================
// Anime Specific
// ============================================================================

export const ANIME_SEASONS = ["Any", "Winter", "Spring", "Summer", "Fall"];
export const ANIME_FORMATS = ["Any", "TV Show", "Movie", "TV Short", "Special", "OVA", "ONA", "Music"];
export const ANIME_STATUS = ["Any", "Airing", "Finished", "Not Yet Aired", "Cancelled"];

export const ANIME_STREAMING_ON = [
	"Crunchyroll", "Hulu", "Netflix", "YouTube", "HIDIVE", "Amazon Prime Video", "Vimeo", 
	"RetroCrush", "Adult Swim", "Japanese Film Archives", "Tubi TV", "Crackle", "AsianCrush", 
	"Midnight Pulp", "Bilibili", "Disney Plus", "Bilibili TV", "Tencent Video", "iQ", "Youku", 
	"WeTV", "Niconico Video", "iQIYI", "Star+", "Max", "Viki", "Cineverse", "Youku TV", 
	"Coolmic", "Criterion Channel", "Hoopla", "Laftel", "OceanVeil", "Apple TV+", 
	"Bandai Channel", "Prime Video"
];


// ============================================================================
// Manga Specific
// ============================================================================

export const MANGA_FORMATS = ["Any", "Manga", "Light Novel", "One Shot"];
export const MANGA_STATUS = ["Any", "Publishing", "Finished", "Not Yet Published", "Cancelled", "Hiatus"];
export const COUNTRY_OF_ORIGIN = ["Any", "Japan", "South Korea", "China", "Taiwan"];

export const MANGA_READABLE_ON = {
	"English": [
		"FAKKU", "WebComics", "MANGA Plus", "WEBTOON", "Toomics", "Lezhin", "Futekiya", 
		"Manga Planet", "Tapas", "Tappytoon", "Manta", "Webnovel", "MangaToon", "J-Novel Club", 
		"Lalatoon", "TOPTOON", "MangaPlaza", "VIZ", "Omoi", "Comikey", "INKR", "Alpha Manga", 
		"Pixiv", "Coolmic", "Lezhin X", "Manga UP!", "Irodori Comics", "K MANGA", "Doujin", "Renta!"
	],
	"Korean": [
		"Lezhin", "Toomics", "Naver Webtoon", "KakaoPage", "Bomtoon", "Naver Series", "TOPTOON", 
		"Kakao Webtoon", "Anytoon", "Mootoon", "Onestory", "QToon"
	],
	"Japanese": [
		"Mangabox", "Kadocomi", "Nico Nico Seiga", "Pixiv Comic", "Comico", "Piccoma", 
		"Shonen Jump Plus", "Pocket Magazine", "Sunday Webry", "Ganma!", "Cycomics", 
		"Tonari no Young Jump", "Manga Park", "Comic Days", "Comic Zenon", "Manga Library Z", 
		"Manga Love", "Sukima", "Manga Dokuha", "AlphaPolis", "Ura Sunday", "Comic Essay", 
		"Lezhin", "Kurage Bunch", "Comic Action", "Lalatoon", "Comic Fuz", "MangaToon", 
		"Champion Cross", "Comiplex", "Comic Meteor", "Comic Ride", "Magcomi", "Gangan Online", 
		"Comic Trail", "Ciao Plus", "Toomics", "BeLTOON", "Yanmaga", "Young Animal", "Big Comics", 
		"Young Champion Web", "Manga UP!", "Flower Comics", "Rimacomi Plus", 
		"Weekly CoroCoro Comic", "Shonen Jump", "TOPTOON", "Manga Mee", "Yawaraka Spirits", 
		"Gau Gau", "Comic Ryu", "Asacomi", "Ichijin Plus", "FEEL web", "Takecomic", "SORAJIMA TOON"
	],
	"Chinese": [
		"Tencent Comics", "KuaiKan Manhua", "Dajiaochong Manhua", "Manman Manhua", "Kai Manhua", 
		"MangaToon", "Creative Comic Collection", "Lalatoon", "MKZhan", "Bilibili", "iQIYI", 
		"WEBTOON", "Zhiyin Manke", "Kanmanhua", "TOPTOON", "Toomics", "Bomtoon", "Dongman Manhua"
	],
	"Spanish": ["Lezhin", "WEBTOON", "MangaToon", "Toomics", "Manta"],
	"Thai": ["MangaToon", "WEBTOON", "Lezhin", "Kakao Webtoon", "WeComics", "Toomics"],
	"French": ["MangaToon", "WEBTOON", "Tappytoon", "Lezhin", "Toomics", "Mangas.io", "ONO"],
	"German": ["WEBTOON", "Tappytoon", "Toomics", "Lezhin"]
};

// Generates Array of Years (e.g., from 1950 to current year + 1)
export function getYearsOffset(past: number = 75, future: number = 1): string[] {
	const currentYear = new Date().getFullYear();
	const years = ["Any"];
	for (let i = currentYear + future; i >= currentYear - past; i--) {
		years.push(i.toString());
	}
	return years;
}
