<script lang="ts">
	/**
	 * Settings panel for notification preferences.
	 * Stored in localStorage.
	 */
	import {
		Card,
		CardContent,
		CardDescription,
		CardHeader,
		CardTitle,
	} from '$lib/components/ui/card';
	import { Label } from '$lib/components/ui/label';
	import { Switch } from '$lib/components/ui/switch';
	import { Button } from '$lib/components/ui/button';
	import Icon from '@iconify/svelte';
	import { browser } from '$app/environment';
	import { toast } from 'svelte-sonner';

	const STORAGE_KEY = 'zafkiel:notification_settings';

	interface NotificationSettingsData {
		// Per-type toggles
		AIRING: boolean;
		ACTIVITY_MESSAGE: boolean;
		ACTIVITY_REPLY: boolean;
		ACTIVITY_MENTION: boolean;
		ACTIVITY_LIKE: boolean;
		ACTIVITY_REPLY_LIKE: boolean;
		ACTIVITY_REPLY_SUBSCRIBED: boolean;
		FOLLOWING: boolean;
		THREAD_COMMENT_MENTION: boolean;
		THREAD_SUBSCRIBED: boolean;
		THREAD_COMMENT_REPLY: boolean;
		THREAD_LIKE: boolean;
		THREAD_COMMENT_LIKE: boolean;
		RELATED_MEDIA_ADDITION: boolean;
		MEDIA_DATA_CHANGE: boolean;
		MEDIA_MERGE: boolean;
		MEDIA_DELETION: boolean;
		// General
		pollingIntervalSeconds: number;
		autoMarkRead: boolean;
		autoMarkReadDelaySeconds: number;
	}

	function defaults(): NotificationSettingsData {
		return {
			AIRING: true,
			ACTIVITY_MESSAGE: true,
			ACTIVITY_REPLY: true,
			ACTIVITY_MENTION: true,
			ACTIVITY_LIKE: true,
			ACTIVITY_REPLY_LIKE: true,
			ACTIVITY_REPLY_SUBSCRIBED: true,
			FOLLOWING: true,
			THREAD_COMMENT_MENTION: true,
			THREAD_SUBSCRIBED: true,
			THREAD_COMMENT_REPLY: true,
			THREAD_LIKE: false,
			THREAD_COMMENT_LIKE: false,
			RELATED_MEDIA_ADDITION: true,
			MEDIA_DATA_CHANGE: false,
			MEDIA_MERGE: false,
			MEDIA_DELETION: true,
			pollingIntervalSeconds: 60,
			autoMarkRead: true,
			autoMarkReadDelaySeconds: 5,
		};
	}

	function loadSettings(): NotificationSettingsData {
		if (!browser) return defaults();
		try {
			const raw = localStorage.getItem(STORAGE_KEY);
			if (raw) return { ...defaults(), ...JSON.parse(raw) };
		} catch {}
		return defaults();
	}

	let settings = $state<NotificationSettingsData>(loadSettings());

	function save() {
		if (browser) {
			localStorage.setItem(STORAGE_KEY, JSON.stringify(settings));
		}
		toast.success('Notification settings saved');
	}

	type NotifGroup = {
		label: string;
		icon: string;
		keys: (keyof NotificationSettingsData)[];
		keyLabels: string[];
	};

	const GROUPS: NotifGroup[] = [
		{
			label: 'Airing',
			icon: 'solar:play-stream-bold-duotone',
			keys: ['AIRING'],
			keyLabels: ['New episode airing'],
		},
		{
			label: 'Activity',
			icon: 'solar:chat-round-bold-duotone',
			keys: [
				'ACTIVITY_MESSAGE',
				'ACTIVITY_REPLY',
				'ACTIVITY_MENTION',
				'ACTIVITY_LIKE',
				'ACTIVITY_REPLY_LIKE',
				'ACTIVITY_REPLY_SUBSCRIBED',
			],
			keyLabels: [
				'New messages',
				'Replies to your activity',
				'Mentions',
				'Likes on activity',
				'Likes on replies',
				'New replies on subscribed activity',
			],
		},
		{
			label: 'Social',
			icon: 'solar:users-group-two-rounded-bold-duotone',
			keys: ['FOLLOWING'],
			keyLabels: ['New followers'],
		},
		{
			label: 'Forums',
			icon: 'solar:chat-square-bold-duotone',
			keys: [
				'THREAD_COMMENT_MENTION',
				'THREAD_SUBSCRIBED',
				'THREAD_COMMENT_REPLY',
				'THREAD_LIKE',
				'THREAD_COMMENT_LIKE',
			],
			keyLabels: [
				'Mentions in threads',
				'New comments on subscribed threads',
				'Replies to your comments',
				'Likes on threads',
				'Likes on comments',
			],
		},
		{
			label: 'Media Updates',
			icon: 'solar:tv-bold-duotone',
			keys: ['RELATED_MEDIA_ADDITION', 'MEDIA_DATA_CHANGE', 'MEDIA_MERGE', 'MEDIA_DELETION'],
			keyLabels: ['Related media added', 'Media data changes', 'Media merges', 'Media deletions'],
		},
	];

	const POLLING_OPTIONS = [
		{ label: '30 seconds', value: 30 },
		{ label: '1 minute', value: 60 },
		{ label: '2 minutes', value: 120 },
		{ label: '5 minutes', value: 300 },
		{ label: '15 minutes', value: 900 },
	];

	const AUTO_READ_DELAY_OPTIONS = [
		{ label: 'Immediately', value: 0 },
		{ label: '3 seconds', value: 3 },
		{ label: '5 seconds', value: 5 },
		{ label: '10 seconds', value: 10 },
	];
</script>

<Card>
	<CardHeader>
		<CardTitle class="flex items-center gap-2">
			<Icon icon="solar:bell-bold-duotone" class="size-5 text-primary" />
			Notification Settings
		</CardTitle>
		<CardDescription>
			Control which notifications you receive and how they're handled.
		</CardDescription>
	</CardHeader>
	<CardContent class="space-y-6">
		<!-- Per-type toggles -->
		{#each GROUPS as group}
			<div>
				<div class="mb-3 flex items-center gap-2">
					<Icon icon={group.icon} class="size-4 text-muted-foreground" />
					<span class="text-sm font-semibold">{group.label}</span>
				</div>
				<div class="space-y-3 pl-6">
					{#each group.keys as key, i}
						<div class="flex items-center justify-between">
							<Label class="cursor-pointer text-sm font-normal">{group.keyLabels[i]}</Label>
							<Switch
								checked={settings[key as keyof NotificationSettingsData] as boolean}
								onCheckedChange={(v) => {
									(settings as unknown as Record<string, unknown>)[key] = v;
								}}
							/>
						</div>
					{/each}
				</div>
			</div>

			{#if group !== GROUPS[GROUPS.length - 1]}
				<div class="border-t"></div>
			{/if}
		{/each}

		<div class="border-t"></div>

		<!-- Polling interval -->
		<div>
			<Label for="polling-interval" class="mb-2 block text-sm font-medium">
				Check for new notifications
			</Label>
			<select
				id="polling-interval"
				bind:value={settings.pollingIntervalSeconds}
				class="w-full rounded-md border bg-background px-3 py-2 text-sm focus:ring-2 focus:ring-ring focus:outline-none"
			>
				{#each POLLING_OPTIONS as opt}
					<option value={opt.value}>{opt.label}</option>
				{/each}
			</select>
		</div>

		<!-- Auto mark read -->
		<div class="space-y-3">
			<div class="flex items-center justify-between">
				<div>
					<Label class="text-sm font-medium">Auto-mark as Read</Label>
					<p class="text-xs text-muted-foreground">
						Mark notifications read after viewing the page
					</p>
				</div>
				<Switch
					checked={settings.autoMarkRead}
					onCheckedChange={(v) => (settings.autoMarkRead = v)}
				/>
			</div>

			{#if settings.autoMarkRead}
				<div>
					<Label for="auto-read-delay" class="mb-2 block text-sm font-medium">
						Delay before marking read
					</Label>
					<select
						id="auto-read-delay"
						bind:value={settings.autoMarkReadDelaySeconds}
						class="w-full rounded-md border bg-background px-3 py-2 text-sm focus:ring-2 focus:ring-ring focus:outline-none"
					>
						{#each AUTO_READ_DELAY_OPTIONS as opt}
							<option value={opt.value}>{opt.label}</option>
						{/each}
					</select>
				</div>
			{/if}
		</div>

		<!-- Save -->
		<Button onclick={save} class="w-full">
			<Icon icon="solar:diskette-bold-duotone" class="mr-2 size-4" />
			Save Notification Settings
		</Button>
	</CardContent>
</Card>
