<script lang="ts">
	import CachedImage from '$lib/components/ui/CachedImage.svelte';
	import Icon from '@iconify/svelte';
	import { Badge } from '$lib/components/ui/badge';

	interface Review {
		id: number;
		summary: string;
		body: string;
		rating: number;
		ratingAmount: number;
		user: {
			id: number;
			name: string;
			avatar: {
				large: string;
				medium: string;
			};
		};
	}

	export let reviews: Review[] = [];
	export let isLoading: boolean = false;

	function stripHtml(html: string) {
		return html.replace(/<[^>]*>/g, '');
	}
</script>

{#if isLoading}
	<div class="flex items-center justify-center p-8">
		<Icon icon="solar:refresh-circle-line-duotone" class="h-8 w-8 animate-spin text-primary" />
	</div>
{:else if reviews.length > 0}
	<div class="grid grid-cols-1 gap-4 md:grid-cols-2">
		{#each reviews as review}
			<div
				class="flex flex-col gap-4 rounded-lg border border-border bg-card p-4 transition-shadow hover:shadow-md"
			>
				<div class="flex items-center justify-between">
					<div class="flex items-center gap-3">
						<a href="/user/{review.user.id}">
							<CachedImage
								src={review.user.avatar.medium}
								alt={review.user.name}
								class="h-10 w-10 rounded-full object-cover"
							/>
						</a>
						<div>
							<a href="/user/{review.user.id}" class="font-semibold hover:text-primary">
								{review.user.name}
							</a>
							<div class="flex items-center gap-2 text-xs text-muted-foreground">
								<span class="flex items-center gap-1">
									<Icon icon="lucide:thumbs-up" class="size-3" />
									{review.rating}
								</span>
							</div>
						</div>
					</div>
				</div>

				<div class="space-y-2">
					<h4 class="font-semibold">{review.summary}</h4>
					<p class="line-clamp-4 text-sm text-muted-foreground">
						{stripHtml(review.body)}
					</p>
				</div>
			</div>
		{/each}
	</div>
{:else}
	<div
		class="rounded-lg border border-dashed border-muted-foreground/30 bg-muted/50 p-8 text-center"
	>
		<Icon icon="lucide:message-square" class="mx-auto mb-3 size-12 text-muted-foreground/50" />
		<p class="text-muted-foreground">No reviews available.</p>
	</div>
{/if}
