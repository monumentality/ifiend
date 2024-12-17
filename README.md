# ifiend  

**Check out the latest videos from your favorite channels in-terminal.**

![videofetch](https://i.giphy.com/media/v1.Y2lkPTc5MGI3NjExMWN0NGV0dnR0aG9oNjFqanY3bW14dXR5eHc1N2VrbWo5MnhwbzV5MiZlcD12MV9pbnRlcm5hbF9naWZfYnlfaWQmY3Q9Zw/R3khsJ6oTW8UNu7cmC/giphy.gif)

Supports thumbnails in-terminal if you're using [kitty terminal](https://github.com/kovidgoyal/kitty).

**HTML file generation for those that don't use kitty terminal.**

![html](https://i.giphy.com/media/v1.Y2lkPTc5MGI3NjExcmxwam9udXNyM2Z1cHdjbjhld2xvbzRzZWwxdHhqMDN0bno2c3JpcSZlcD12MV9pbnRlcm5hbF9naWZfYnlfaWQmY3Q9Zw/99d0uN46SSZxnSwBlB/giphy.gif)

**Select and download videos via [yt-dlp](https://github.com/yt-dlp/yt-dlp) or downloader of your choice.**

![download](https://i.giphy.com/media/v1.Y2lkPTc5MGI3NjExMjN6OXJjcGloMWVmZXpuM2xweG9mZXkzcjJrbTAwY2loMHR0OWVrYiZlcD12MV9pbnRlcm5hbF9naWZfYnlfaWQmY3Q9Zw/UKn0QzT7W1T2LGBpWS/giphy.gif)

yt-dlp by default, can change in `~/.config/config.toml` 
# installation

### Download pre-compiled

1. Navigate to [Releases](https://github.com/monumentality/ifiend/releases) tab and download the latest version.
2. Grant the downloaded binary permission to execute: `chmod +x ifiend`
### Build from source

1. Install the Rust toolchain for your distro.
2. Run `git clone https://github.com/monumentality/ifiend.git` or download and unpack the source code manually.
3. Navigate to the `ifiend` directory.
4. Run `cargo build --release`
5. Wait for ifiend to compile.
6. Now you can run `cargo run --release --` or head straight to `target/release` directory and grab your ifiend executable from there.

# usage

**Get help:**
```bash
> ifiend
```

**Fetch recent videos from default channels:** 
```bash
> ifiend fetch
```

  Gets default channels and videos-per-channel from your `config.toml`. Generates the config if you don't already have one. Once generated, you can change it in `~/.config/ifiend/config.toml`


**Fetch [vpc] videos from each channel.** 
```bash
> ifiend fetch [vpc]
```
Replace [vpc] with desired number of videos.

**Examples**:

```bash
> ifiend fetch 1
```

**Fetch [vpc] videos from [channel].** 
```bash
> ifiend fetch [vpc] [channel] [channel2] [...]
```
Replace [vpc] and [channel] with desired number of videos and channels to get those videos from, respectively.

  You can provide multiple channels. You can also provide incomplete channel handles and ifiend will scan your config and try to parse your partial handle into a full one. These are case-sensitive though. For example, if you type `ifiend fetch 1 Oney` and have `@OneyPlays` in your `config.toml` ifiend will understand your argument as `@OneyPlays`.


**Examples:**

Get the 4 latest videos from @hbomberguy. 
```bash
> ifiend fetch 4 @hbomberguy
```

Does the same if your `config.toml` has `@hbomberguy` in it.
```bash
> ifiend fetch 4 hbomb
```

Same thing.
```bash
> ifiend fetch 4 guy
```

