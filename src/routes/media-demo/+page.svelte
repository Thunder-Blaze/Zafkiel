<script lang="ts">
	import type { MediaData } from '$lib/types/media';
	import MediaCard from '$lib/components/MediaCard.svelte';
	import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '$lib/components/ui/card';
	import { Badge } from '$lib/components/ui/badge';
	import Icon from '@iconify/svelte';
	import { fade, slide } from 'svelte/transition';

	// Sample Anime Data
	const sampleAnime: MediaData[] = [
		{
			id: 1,
			title: 'Steins;Gate',
			englishTitle: 'Steins Gate',
			coverImage: 'https://s4.anilist.co/file/anilistcdn/media/anime/cover/large/bx9253-7pdcVzQSkKxT.jpg',
			bannerImage: 'https://s4.anilist.co/file/anilistcdn/media/anime/banner/9253-SL1BSco8npDZ.jpg',
			score: 9.1,
			status: 'FINISHED',
			userStatus: 'COMPLETED',
			userProgress: 24,
			totalEpisodes: 24,
			genres: ['Sci-Fi', 'Thriller', 'Drama'],
			studio: 'White Fox',
			year: 2011,
			type: 'ANIME',
			format: 'TV',
			season: 'SPRING',
			isAdult: false,
			description: 'A group of friends have customized their microwave into a device that can send text messages to the past. As they perform different experiments, an organization named SERN who has been doing their own research on time travel tracks them down.',
			duration: 24,
			popularity: 250000,
			favourites: 85000
		},
		{
			id: 2,
			title: '進撃の巨人',
			englishTitle: 'Attack on Titan',
			coverImage: 'https://s4.anilist.co/file/anilistcdn/media/anime/cover/large/bx16498-C6FPmWm59CyP.jpg',
			bannerImage: 'https://s4.anilist.co/file/anilistcdn/media/anime/banner/16498-8jpFCOcDmneX.jpg',
			score: 8.5,
			status: 'FINISHED',
			userStatus: 'CURRENT',
			userProgress: 45,
			totalEpisodes: 75,
			genres: ['Action', 'Drama', 'Fantasy', 'Mystery'],
			studio: 'Wit Studio',
			year: 2013,
			type: 'ANIME',
			format: 'TV',
			season: 'SPRING',
			isAdult: false,
			description: 'Centuries ago, mankind was slaughtered to near extinction by monstrous humanoid creatures called titans, forcing humans to hide in fear behind enormous concentric walls.',
			duration: 24,
			popularity: 500000,
			favourites: 120000
		},
		{
			id: 3,
			title: 'コードギアス 反逆のルルーシュ',
			englishTitle: 'Code Geass: Lelouch of the Rebellion',
			coverImage: 'https://s4.anilist.co/file/anilistcdn/media/anime/cover/large/bx1575-2sVh6JhxJ4ka.jpg',
			bannerImage: 'https://s4.anilist.co/file/anilistcdn/media/anime/banner/1575.jpg',
			score: 8.7,
			status: 'FINISHED',
			userStatus: 'PLANNING',
			userProgress: 0,
			totalEpisodes: 25,
			genres: ['Action', 'Drama', 'Mecha', 'Sci-Fi'],
			studio: 'Sunrise',
			year: 2006,
			type: 'ANIME',
			format: 'TV',
			season: 'FALL',
			isAdult: false,
			description: 'The Empire of Britannia has invaded Japan using giant robot weapons called Knightmare Frames. Japan is now referred to as Area 11.',
			duration: 24,
			popularity: 350000,
			favourites: 95000
		},
		{
			id: 4,
			title: 'Death Note',
			coverImage: 'https://s4.anilist.co/file/anilistcdn/media/anime/cover/large/bx1535-4r88a1tsBEIz.jpg',
			bannerImage: 'https://s4.anilist.co/file/anilistcdn/media/anime/banner/1535.jpg',
			score: 8.6,
			status: 'FINISHED',
			userStatus: 'COMPLETED',
			userProgress: 37,
			totalEpisodes: 37,
			genres: ['Mystery', 'Psychological', 'Supernatural', 'Thriller'],
			studio: 'Madhouse',
			year: 2006,
			type: 'ANIME',
			format: 'TV',
			season: 'FALL',
			isAdult: false,
			description: 'A high school student discovers a supernatural notebook that allows him to kill anyone by writing their name while picturing their face.',
			duration: 23,
			popularity: 450000,
			favourites: 110000
		}
	];

	// Sample Manga Data
	const sampleManga: MediaData[] = [
		{
			id: 5,
			title: 'Berserk',
			coverImage: 'https://s4.anilist.co/file/anilistcdn/media/manga/cover/large/bx30002-7DyMZxvzWKC8.jpg',
			bannerImage: 'https://s4.anilist.co/file/anilistcdn/media/manga/banner/30002-3TuoOKe0fLuE.jpg',
			score: 9.4,
			status: 'RELEASING',
			userStatus: 'CURRENT',
			userProgress: 200,
			totalEpisodes: 377,
			genres: ['Action', 'Adventure', 'Drama', 'Fantasy', 'Horror'],
			studio: 'Hakusensha',
			year: 1989,
			type: 'MANGA',
			format: 'MANGA',
			isAdult: true,
			description: 'Guts, a former mercenary now known as the "Black Swordsman," is out for revenge. After a tumultuous childhood, he finally finds someone he respects and believes he can trust, only to have everything fall apart when this person takes away everything important to Guts.',
			popularity: 180000,
			favourites: 75000
		},
		{
			id: 6,
			title: 'One Piece',
			coverImage: 'https://s4.anilist.co/file/anilistcdn/media/manga/cover/large/bx30013-ulXvn0lzWvsz.jpg',
			bannerImage: 'https://s4.anilist.co/file/anilistcdn/media/manga/banner/30013-wWcVqZ2j33Ag.jpg',
			score: 9.2,
			status: 'RELEASING',
			userStatus: 'CURRENT',
			userProgress: 850,
			totalEpisodes: 1100,
			genres: ['Action', 'Adventure', 'Comedy', 'Fantasy'],
			studio: 'Shueisha',
			year: 1997,
			type: 'MANGA',
			format: 'MANGA',
			isAdult: false,
			description: 'Monkey D. Luffy refuses to let anyone or anything stand in the way of his quest to become king of all pirates. With a course charted for the treacherous waters of the Grand Line, this is one captain who\'ll never drop anchor until he\'s claimed the greatest treasure on Earth: the Legendary One Piece!',
			popularity: 420000,
			favourites: 150000
		},
		{
			id: 7,
			title: 'Vagabond',
			coverImage: 'https://s4.anilist.co/file/anilistcdn/media/manga/cover/large/bx30656-6Jm0yUVXTQsP.jpg',
			bannerImage: 'https://s4.anilist.co/file/anilistcdn/media/manga/banner/30656-yhhOeMVI4IDr.jpg',
			score: 9.1,
			status: 'RELEASING',
			userStatus: 'PAUSED',
			userProgress: 150,
			totalEpisodes: 327,
			genres: ['Action', 'Adventure', 'Drama'],
			studio: 'Kodansha',
			year: 1998,
			type: 'MANGA',
			format: 'MANGA',
			isAdult: false,
			description: 'Growing up in the late 16th century Shinmen Takezou is shunned by the local villagers as a devil child due to his wild and violent nature. Running away from home with a fellow boy at age 17, Takezo joins the Toyotomi army to fight the Tokugawa clan at the Battle of Sekigahara.',
			popularity: 95000,
			favourites: 42000
		}
	];

	// Sample Adult Content
	const sampleAdult: MediaData[] = [
		{
			id: 8,
			title: 'Yosuga no Sora',
			coverImage: 'https://s4.anilist.co/file/anilistcdn/media/anime/cover/large/bx8861-RJzoGhS3gWEj.png',
			bannerImage: 'https://s4.anilist.co/file/anilistcdn/media/anime/banner/8861-U9RVyRifjIEP.jpg',
			score: 6.5,
			status: 'FINISHED',
			userStatus: 'DROPPED',
			userProgress: 5,
			totalEpisodes: 12,
			genres: ['Drama', 'Ecchi', 'Romance'],
			studio: 'feel.',
			year: 2010,
			type: 'ANIME',
			format: 'TV',
			season: 'FALL',
			isAdult: true,
			description: 'Haruka and Sora Kasugano are coming home, to a place filled with memories.',
			duration: 24,
			popularity: 85000,
			favourites: 15000
		},
		{
			id: 9,
			title: 'Prison School',
			coverImage: 'https://s4.anilist.co/file/anilistcdn/media/manga/cover/large/bx25490-IYlklBdRdb7N.jpg',
			bannerImage: 'https://s4.anilist.co/file/anilistcdn/media/manga/banner/25490-4gFdDG1uH4Q0.jpg',
			score: 8.6,
			status: 'FINISHED',
			userStatus: 'COMPLETED',
			userProgress: 277,
			totalEpisodes: 277,
			genres: ['Comedy', 'Ecchi', 'Romance'],
			studio: 'Kodansha',
			year: 2011,
			type: 'MANGA',
			format: 'MANGA',
			isAdult: true,
			description: 'Hachimitsu Academy, once an all-girls school, has become co-ed, and teen Kiyoshi is one of five boys to enroll. When he is caught peeping, Kiyoshi is sent to the schools prison, where his punishment is carried out.',
			popularity: 120000,
			favourites: 35000
		}
	];

	// Sample items with various statuses
	const sampleStatuses: MediaData[] = [
		{
			id: 10,
			title: 'Currently Watching',
			coverImage: 'https://s4.anilist.co/file/anilistcdn/media/anime/cover/large/bx21459-RoPwgrZ32gM3.jpg',
			score: 8.8,
			status: 'RELEASING',
			userStatus: 'CURRENT',
			userProgress: 10,
			totalEpisodes: 24,
			genres: ['Action', 'Drama'],
			type: 'ANIME',
			format: 'TV',
			isAdult: false,
			description: 'A show you are currently watching.'
		},
		{
			id: 11,
			title: 'Plan to Watch',
			coverImage: 'https://s4.anilist.co/file/anilistcdn/media/anime/cover/large/bx20785-Jw45NIZGud3j.jpg',
			score: 8.5,
			status: 'FINISHED',
			userStatus: 'PLANNING',
			userProgress: 0,
			totalEpisodes: 12,
			genres: ['Comedy', 'Slice of Life'],
			type: 'ANIME',
			format: 'TV',
			isAdult: false,
			description: 'A show on your plan to watch list.'
		},
		{
			id: 12,
			title: 'Dropped',
			coverImage: 'https://s4.anilist.co/file/anilistcdn/media/anime/cover/large/bx99423-NpW9H8IyFlLN.jpg',
			score: 6.2,
			status: 'FINISHED',
			userStatus: 'DROPPED',
			userProgress: 3,
			totalEpisodes: 12,
			genres: ['Fantasy', 'Adventure'],
			type: 'ANIME',
			format: 'TV',
			isAdult: false,
			description: 'A show you dropped.'
		},
		{
			id: 13,
			title: 'On Hold/Paused',
			coverImage: 'https://s4.anilist.co/file/anilistcdn/media/anime/cover/large/bx11757-SjVojHqp15kF.jpg',
			score: 7.8,
			status: 'FINISHED',
			userStatus: 'PAUSED',
			userProgress: 8,
			totalEpisodes: 22,
			genres: ['Mystery', 'Supernatural'],
			type: 'ANIME',
			format: 'TV',
			isAdult: false,
			description: 'A show you paused.'
		},
		{
			id: 14,
			title: 'Completed',
			coverImage: 'https://s4.anilist.co/file/anilistcdn/media/anime/cover/large/bx21087-sHZNbp1M6PfP.jpg',
			score: 9.0,
			status: 'FINISHED',
			userStatus: 'COMPLETED',
			userProgress: 12,
			totalEpisodes: 12,
			genres: ['Comedy', 'Supernatural'],
			type: 'ANIME',
			format: 'TV',
			isAdult: false,
			description: 'A completed show.'
		},
		{
			id: 15,
			title: 'Rewatching',
			coverImage: 'https://s4.anilist.co/file/anilistcdn/media/anime/cover/large/bx20958-t6FXPMgW45iF.jpg',
			score: 8.9,
			status: 'FINISHED',
			userStatus: 'REPEATING',
			userProgress: 5,
			totalEpisodes: 25,
			genres: ['Action', 'Sci-Fi'],
			type: 'ANIME',
			format: 'TV',
			isAdult: false,
			description: 'A show you are rewatching.'
		}
	];
</script>

<div class="container mx-auto max-w-7xl space-y-8 p-6">
	<!-- Header -->
	<div class="space-y-2" transition:slide={{ duration: 300 }}>
		<div class="flex items-center gap-3">
			<div class="rounded-lg bg-primary/10 p-2">
				<Icon icon="solar:gallery-bold" class="h-6 w-6 text-primary" />
			</div>
			<h1 class="text-4xl font-bold">Media Card Demo</h1>
		</div>
		<p class="text-lg text-foreground/70">
			Showcase of MediaCard component with various types of anime and manga
		</p>
	</div>

	<!-- Popular Anime Section -->
	<section class="space-y-4" transition:slide={{ duration: 300, delay: 100 }}>
		<Card>
			<CardHeader>
				<div class="flex items-center gap-3">
					<div class="flex h-10 w-10 items-center justify-center rounded-xl bg-blue-500/10">
						<Icon icon="solar:tv-bold" class="h-5 w-5 text-blue-500" />
					</div>
					<div>
						<CardTitle>Popular Anime</CardTitle>
						<CardDescription>
							Highly rated anime series with various statuses
						</CardDescription>
					</div>
				</div>
			</CardHeader>
			<CardContent>
				<div class="grid gap-6 sm:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6">
					{#each sampleAnime as anime (anime.id)}
						<MediaCard
							mediaData={anime}
						/>
					{/each}
				</div>
			</CardContent>
		</Card>
	</section>

	<!-- Manga Section -->
	<section class="space-y-4" transition:slide={{ duration: 300, delay: 150 }}>
		<Card>
			<CardHeader>
				<div class="flex items-center gap-3">
					<div class="flex h-10 w-10 items-center justify-center rounded-xl bg-green-500/10">
						<Icon icon="solar:book-2-bold" class="h-5 w-5 text-green-500" />
					</div>
					<div>
						<CardTitle>Popular Manga</CardTitle>
						<CardDescription>
							Top-rated manga series including ongoing releases
						</CardDescription>
					</div>
				</div>
			</CardHeader>
			<CardContent>
				<div class="grid gap-6 sm:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6">
					{#each sampleManga as manga (manga.id)}
						<MediaCard
							mediaData={manga}
						/>
					{/each}
				</div>
			</CardContent>
		</Card>
	</section>

	<!-- Adult Content Section -->
	<section class="space-y-4" transition:slide={{ duration: 300, delay: 200 }}>
		<Card class="border-red-500/20 bg-red-500/5">
			<CardHeader>
				<div class="flex items-center gap-3">
					<div class="flex h-10 w-10 items-center justify-center rounded-xl bg-red-500/10">
						<Icon icon="solar:danger-triangle-bold" class="h-5 w-5 text-red-500" />
					</div>
					<div>
						<CardTitle class="text-red-500">Adult Content (18+)</CardTitle>
						<CardDescription>
							Media with adult content and 18+ badge display
						</CardDescription>
					</div>
					<Badge variant="destructive" class="ml-auto">18+</Badge>
				</div>
			</CardHeader>
			<CardContent>
				<div class="grid gap-6 sm:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6">
					{#each sampleAdult as adult (adult.id)}
						<MediaCard
							mediaData={adult}
						/>
					{/each}
				</div>
			</CardContent>
		</Card>
	</section>

	<!-- Status Variations Section -->
	<section class="space-y-4" transition:slide={{ duration: 300, delay: 250 }}>
		<Card>
			<CardHeader>
				<div class="flex items-center gap-3">
					<div class="flex h-10 w-10 items-center justify-center rounded-xl bg-purple-500/10">
						<Icon icon="solar:list-check-bold" class="h-5 w-5 text-purple-500" />
					</div>
					<div>
						<CardTitle>Different List Statuses</CardTitle>
						<CardDescription>
							Examples of all possible user list statuses
						</CardDescription>
					</div>
				</div>
			</CardHeader>
			<CardContent>
				<div class="grid gap-6 sm:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6">
					{#each sampleStatuses as status (status.id)}
						<MediaCard
							mediaData={status}
						/>
					{/each}
				</div>
			</CardContent>
		</Card>
	</section>

	<!-- Feature Information -->
	<section class="space-y-4" transition:slide={{ duration: 300, delay: 300 }}>
		<Card>
			<CardHeader>
				<div class="flex items-center gap-3">
					<div class="flex h-10 w-10 items-center justify-center rounded-xl bg-amber-500/10">
						<Icon icon="solar:info-circle-bold" class="h-5 w-5 text-amber-500" />
					</div>
					<div>
						<CardTitle>MediaCard Features</CardTitle>
						<CardDescription>
							Hover over cards to see the preview overlay
						</CardDescription>
					</div>
				</div>
			</CardHeader>
			<CardContent class="space-y-4">
				<div class="grid gap-4 md:grid-cols-4">
					<div class="space-y-2">
						<h4 class="font-semibold">Normal Card</h4>
						<ul class="space-y-1 text-sm text-muted-foreground">
							<li>• Cover image with optional glow effect</li>
							<li>• Rating badge with score</li>
							<li>• 18+ badge for adult content</li>
							<li>• Title with 2-line clamp</li>
							<li>• Progress bar for tracking</li>
						</ul>
					</div>
					<div class="space-y-2">
						<h4 class="font-semibold">Preview Card (Hover)</h4>
						<ul class="space-y-1 text-sm text-muted-foreground">
							<li>• Banner image with glow effect</li>
							<li>• Type, format, year badges</li>
							<li>• Genres list</li>
							<li>• Scrollable description</li>
							<li>• Progress controls or status buttons</li>
						</ul>
					</div>
				</div>
				<div class="rounded-lg border border-border/50 bg-muted/30 p-4">
					<h4 class="mb-2 font-semibold">Configurable Effects</h4>
					<p class="text-sm text-muted-foreground">
						Go to <strong>Settings → General → Interface</strong> to toggle animations, glow effects,
						and blur effects. These settings affect all MediaCard displays throughout the app.
					</p>
				</div>
			</CardContent>
		</Card>
	</section>
</div>
