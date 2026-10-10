# Changelog

All notable changes to this project are documented in this file.

Based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/).

## [Unreleased]

### Features

- Run Talk on Debian with Vulkan acceleration

Talk now runs on Debian under X11 and can use Vulkan GPUs for local transcription. A Debian package
can be built locally; global shortcuts and text entry require an X11 session.

## [0.12.0] - 2026-10-08

### Fixes

- (overlay) No count flashes on the overlay at the end of a single dictation

As a recording was let go, the overlay showed a small "1" with its ring for a moment, as if
another dictation were waiting behind it, when it was that very dictation starting to be
transcribed. The count now only shows the dictations that were already running when the
recording started.

- (overlay) The Windows style no longer leaves a paler, empty card behind

When the Windows style arrived, and again once the text was pasted, the card showed for a moment
with nothing on it and in a lighter grey than its own, then went. It now keeps its shade from the
first moment to the last.

- (overlay) The Windows style has the volume flyout's own proportions

The card was the system's size, and what it held was not where the system puts it. The icon now
sits 13 pixels from the left, the slider is 110 pixels long and starts where the volume's does,
and the timer ends 10 pixels from the right, both a pixel above the middle as on the flyout. A
timer being wider than a volume, the slider gives it four pixels, and keeps that length while it
transcribes.

### Features

- (window) Sort the recording overlay settings by what they apply to

The tab now has a Common card for what the four styles share (background, arrival, reaction to the
voice, timer, text at the end), a card named after the style you picked with only the settings
that style has, and then Position. The less used ones, how long the confirmation stays and the
width of the Windows card's middle, sit under a closed Advanced row that shows how many of them
are not at their defaults. The Windows style now has a Shadow setting too.

- (window) Put the theme actions in the Themes card header and fold the base colours

Going back to the theme, resetting the appearance and saving the theme are three icon buttons in
the header of the Themes card, each with its name on hover. The Base colours card is closed until
you click it, and shows how many of its colours differ from the theme's.

- (transcription) Say what a change of backend or graphics card costs before it is applied

Picking CPU, Vulkan or another graphics card now opens a question that names the choice. It says
that the model is unloaded and loaded again, that dictation is not available meanwhile, and that
the CPU is noticeably slower or that a card's memory will be used. With no model loaded it says
the choice only takes effect at the next load. Cancel, or Escape, leaves everything as it was.

- (overlay) Give the Windows style three fixed zones and a movement between its phases

The card keeps the same three places from start to end: the icon on the left, a zone the width of
the slider in the middle, and a zone on the right for the timer. While it transcribes, the bar
stands exactly where your voice was, and a small ring turns on the right in place of a percentage,
which only flickered there. The bar sweeps, then turns into the progress instead of being swapped
for it. Between two phases the icon swells, the middle opens out from a line, the right fades in,
and the mark at the end arrives with a small bounce.

- (overlay) Redraw the slider and the spectrum of the Windows style

Slider is now the system's own slider, one bar that fills as far as your voice is loud, instead of
a row of segments. Spectrum has a drawing of its own again: fine bars across the whole slider, the
same on both sides of the middle. None of the four drawings stands taller than sixteen pixels.

- (overlay) The overlay's background follows the theme, and has an opacity

The Background setting of the Recording overlay tab is now two, for the four styles alike. The
first says what the overlay is drawn on: like the theme, which is light under a light theme of
Talk and dark under a dark one, or always dark, or always light. The second is a Background
opacity, from 0 to 100 %: under a hundred, what is behind the overlay shows through it, on light
as well as on dark. On the Windows style it is how much the card adds to the system's blur. A
background set to Glass before is dark at 50 % now, one set to Light stays light, and one left on
Dark follows the theme. A PC still on 0.11.0 and signed in to the same account keeps reading the
look: it shows the nearest background it has.

- (overlay) The preview shows the Windows style as it will be

In the Recording overlay tab, the Windows style was drawn as a flat grey card, which said nothing
of the blurred material it has on the desktop. The preview and the style's tile now blur what is
behind the card and lay the same shade over it, so the tone and the opacity you pick look there
as they will.

- (overlay) The shadow under the overlay can be set

Halo and Capsule always cast the same soft shadow. A Shadow setting in the Recording overlay tab
takes it from none, where the overlay lies flat on the screen, to twice what it was, where it
looks raised above it.

- (overlay) The arrival of the overlay has a side

A bounce and a slide came from the edge of the screen the overlay sits by, and left towards it.
A From setting beside Appearance now picks the side: the top, the bottom, the left or the right,
or Auto, which is what it did. It holds for the four styles and for an overlay following the
mouse to another screen. A fade has no side.

- (dashboard) The dashboard names what a person averages, and a measured speed can go back to it

The Your typing card showed your speed at the keyboard and by voice, and nothing to hold them
against. It now also says what a person averages, 40 words a minute at a keyboard and 150 by
voice. A typing speed nobody measured is said to be that average, and one you measured can be
put back to it.

- (overlay) The overlay can follow the mouse from screen to screen

The Screen setting has a rule more: "Where the mouse is, following it". The overlay still
appears on the screen holding the pointer, and when the pointer goes to another screen while it
is shown, it leaves the one and arrives on the other, the way its Appearance setting says.

- (window) A changed setting shows the way back to its default

On every settings page, a setting that is no longer at its default shows a small Default beside
its label, which puts it back. Nothing shows beside a setting nobody touched. A colour or a
shape of the theme goes back to what the theme it started from gives. The shortcuts, the
server's address and token and the graphics card have none.

- (overlay) The Windows style has a width, and so has its middle

The card of the Windows style was the system's 192 pixels and nothing else. Two settings in the
Recording overlay tab now say how wide the card is, from 72 to 240 pixels, and how wide what
draws your voice in its middle is. The spectrum keeps whole bars at any width. A card made too
narrow to keep the icon and the timer beside its middle shows the middle alone, and their
switches say there is no room. With the timer or the microphone icon switched off, the middle
keeps its width and sits in the room they leave.

- (overlay) The icon and the ring shown while it transcribes can be left out

While it transcribes, Halo shows a small brain, and the Windows style a brain and a turning
ring. Neither says anything the bar does not. A Transcription icon switch in the Recording
overlay tab leaves them out, and the bar stays where it was.

- (overlay) The overlay on screen follows a setting as it is moved

With a recording running, the overlay on screen took a change made in the Recording overlay tab
only once the slider was let go. It is now redrawn at each step, like the preview beside it.

- (overlay) The Windows style arrives like the others

The Windows style could only appear at once and fade what it holds, whatever Appearance was set
to. Its card now arrives and leaves whole, the blurred material with it: Slide brings it in from
the edge it sits by, Bounce lets it go a little past its place and come back, Fade leaves it
where it is. The Appearance setting is offered to it like to the three other styles.

- (overlay) The Windows style can be drawn in a palette

The Windows style drew in the accent colour of Windows and nothing else. That colour is now one
palette among the others, named Windows and worn by default: pick the application's, one of the
six or a colour of your own, and the slider, the voice and the ring take it. That style draws in
one colour, so for it the setting is an Accent colour: each chip shows the one colour it gives,
and a colour of your own is one swatch and not three.

- (overlay) The Windows style writes in the application's typeface

The timer and the words of the Windows style are set in Geist, like the rest of Talk, in the room
the system's figures took. Its icons, its ring and its figures now share one weight of line, the
thin one of the system's own icons, where the icon shown while it transcribes was drawn thicker
than the microphone.

### Refactoring

- (window) The grain switch is gone from the Appearance page

The Atmosphere card no longer offers a grain. At the strength it had, the fine texture it laid over
the window could hardly be seen. A theme that had it on simply shows without it, and its other
values are kept as they were.

- (server) The server timeout setting is gone

The Engine page no longer offers a timeout for the server. It only ever covered the time given to a
server to answer a connection, and one that has not answered in three seconds will not, so Talk now
gives up after that, and the dictation goes to the local model when the fallback is on. The time a
transcription may take once the server is reached is unchanged: it still follows the length of the
recording.

## [0.11.0] - 2026-10-06

### Features

- (window) The first-run setup is redrawn in the look of the rest of the application

Every step of the setup now uses the theme you have: choice cards with a ring and a check, a header
with a progress bar that fills as you go, status tiles, the ambient lights behind, and pages that
fade and rise into one another. The window buttons are there from the first step. While a model
downloads, the other models and the Quantised and Standard tabs hold still. A download that fails
now says so and offers to try again, instead of quietly going back to the button, and the last
page lists "Start minimised" beside "Start with Windows".

- (preferences) Meeting mode sits higher on the Dictation page

The Meeting mode card was the last one on the Dictation page, out of sight until you scrolled to the
bottom. It now comes third, under the shortcuts and the recording mode.

- (overlay) Add a style that looks like the Windows volume flyout

The Recording overlay tab has a fourth style, Windows: the card the system shows when you press a
volume key, in the same place, at the same size and on the same kind of blurred background, drawn in
your Windows accent colour. Your voice is shown where the slider would be, as a scrolling wave, a
spectrum, a slider cut into segments or the halo's bars, and the timer sits where the volume figure
would be. A thin bar shows the progress while the text is transcribed. The styles are shown two to a
row.

- (overlay) Choose how long the overlay confirms a paste

The Movement card has a setting for how long the overlay stays once the text is pasted, from nothing
to three seconds. At zero it leaves as soon as the text is in place.

- (overlay) Make the orb watch your pointer and move more with your voice

The orb's eyes follow the mouse wherever it is on the desktop while it listens and thinks. It moves
more at the same setting, the light behind it is much fainter, and the reaction to your voice now
goes up to 250 % for every style.

- (overlay) Let the overlay leave the way it came

The overlay used to vanish at once when a dictation ended. It now fades out, slides away or shrinks,
whichever way it arrived, in a fifth of a second.

- (window) Put the name on the line of the window buttons, which stay on the right

Talk's name sits in the strip at the top of the window, level with the window buttons, with the
sidebar open or folded, and the menu starts higher. The choice of the side the window buttons are on
is gone from Appearance: they are on the right.

### Bug Fixes

- (audio) Meeting mode follows the microphone you chose, and says when it is silent

Meeting mode used to send the Windows default microphone to VB-Cable whatever microphone was chosen
in Settings, so a meeting could hear one microphone while Talk dictated from another. It now sends
the one Talk records from, moves with it when you pick another or when Windows changes its default,
stays silent when that happens in the middle of a dictation, and its card names it. When the
microphone cannot reach the cable, at start-up or because a device went away, a meeting listening to
the cable hears nothing at all: Talk now says so at the foot of the window and on the card, with a
button to try again, instead of leaving the switch looking off. A microphone that is the cable's own
output is refused, since it would feed the cable back into itself.

- (overlay) Stop the overlay taking the keyboard when it is clicked

Clicking the overlay to move it no longer shows a text cursor in it, and the next dictation is typed
where you were writing rather than into the overlay.

- (overlay) Open the hand again after a click on the overlay

The pointer stayed a closed hand over the overlay after a click, until it was next moved.

- (window) Make the Settings and account pages arrive like the others

Going to Settings or to the account page showed their cards at once instead of fading them in. The
audio devices, meeting mode and chained dictations cards were drawn a second time as soon as their
values were read, the account card only appeared once the account had answered, and listing the
audio devices held the window still while the page was arriving.

- (window) Give fields, lists and outlined buttons their background

Text fields, drop-down lists and outlined buttons had no background of their own, so the lights
behind the page showed through them. They now stand on the same surface as the cards.

- (window) Draw the line under a card's introduction across the whole card

The line under the introduction of a card stopped where the text did, short of the card's edge.

- (dashboard) Keep the title of the typing card on one line

The button that runs the typing test again is now a small icon beside the title. As a word it dropped
under the title in a narrow window.

- (preferences) Refuse a Google sign-in that was not given access to Drive

Google's screen offers the access to Google Drive as a box to tick, and lets the sign-in through
without it. Talk then looked signed in and every sync failed with a technical message. Such a sign-in
is now refused at once, saying which box to tick.

## [0.10.0] - 2026-10-05

### Features

- (window) Draw the sidebar and the top strip as one surface

The line between the sidebar and the strip that carries the window buttons is gone: the two are one
plain surface, and the page sits in their corner with a rounded edge. The name of the application
stays when the sidebar is folded, centred above the icons.

- (preferences) List your devices on the Account page

When you are signed in, the Account page shows every PC on the account with the time it saved you
and either its dictations (this PC) or when it was last seen, and each one can be renamed from the
page. A PC shows up as soon as it signs in, even before it has dictated anything, and drops off the
list after three months without a sign of life. New PCs start with the name of the computer.

- (preferences) Write everything in Geist

The whole window, the overlay included, now uses Geist, and nothing is set in a monospace face
anymore: the timer, the counters, the server addresses, the ports and the pairing code keep their
digits the same width, so they line up.

- (preferences) Make the colours and the spacing less dull

The thumb of a switch is white whether it is on or not, and text on a button takes white or dark
depending on which the gradient carries with the lighter touch, so buttons keep their brightness.
Secondary text is dimmer, the green of a working engine is a little greener, and text takes the
typeface's own line height, so cards are no longer padded with air.

- (preferences) Give buttons and menus a little more height

Buttons and drop-down menus are two pixels taller, the theme previews and the sidebar entries follow
the corner roundness of the theme instead of a fixed one.

- (window) Say in the sidebar what is wrong with the engine

The pill above the bottom links now says the server is unreachable even when no local model is
loaded either, and says the fallback has no model when the server answers but nothing is there to
fall back on. Several problems at once show the most serious one.

- (dashboard) Open on the time you won

The dashboard starts with the period filters, then one large card with the time won in big gradient
figures, the sentence under it, and the days, the streak, the best day and the share run locally
beside it, stacking under the figure when the window is narrow. The four figures come next, then the
year of activity, then the hosted APIs, the subscriptions and your typing side by side, with the
typing test button in the header of the last. The band that said Ready, the shortcut and the model
name is gone: the sidebar pill says when the engine needs attention.

- (window) Pages start at the top

The Engine, Dictation, Appearance, Settings, History, Vocabulary and Account pages no longer open
with their name and a sentence, since the sidebar already says where you are.

- (window) Give the controls one shape

Switches stretch while you press them and land with a small overshoot, menus open on a field whose
arrow turns and a list that pops, buttons are the same height everywhere with the accent gradient on
the main one, sliders show their gradient fill with a white handle, and the chosen card of a group
wears the gradient as its outline.

- (window) Tidy the caption strip and the sidebar

The window buttons in the strip are drawn smaller, the sidebar's name and its first link sit lower,
the status pill is taller, the account entry shows a dot with the sync state, and with the buttons on
the left and the sidebar folded the three dots now stand in a column with room around them.

- (history) Keep the retention beside the count

The number of transcriptions to keep is a visible menu next to the clear button, and each entry shows
its time, its date and where it was run on one line.

- (preferences) Add vocabulary terms from their own card

The Vocabulary page is two cards: one with the field, the Add button and a hint to add terms, and one
listing them with their count next to the title and Clear all on the right. The page scrolls as a
whole instead of the list scrolling inside its card.

- (preferences) Order the Engine page like the others

Models come first, then acceleration, then sharing. The Local and Server choice is a full-width
switch at the top, the graphics card is picked from a menu, and each model is a row with its action on
the right.

- (preferences) Lay Dictation and Settings out as rows

Each setting is a bold name with a muted line under it and its control on the right, separated by
hairlines inside one card per subject. Shortcuts are rows with their keys, the chained dictation
choices are menus, and the interface language moved into the System card.

- (preferences) Choose the overlay's style, colours, movement and position

The overlay tab of Appearance is rebuilt around a live preview on a fake desktop, which loops through
recording, transcribing, pasted and refused or holds on the one you pick. Three styles: Halo, today's
overlay refined; Capsule, a dark pill that stretches with your voice scrolling by as a wave and
shimmering text while it transcribes; and Orb, your account's avatar, which swells with your voice and
sends out ripples, thinks inside a spinning ring, smiles and hops when the text is pasted and scowls
when a dictation is refused. Colours come from the application's accent gradient, from one of the six
overlay themes you already had (yours stays selected), or from three colours of your own, on a dark,
glass or light background. You can set how strongly it reacts to your voice, how it appears (bounce,
slide or fade), its size, the timer, the microphone icon and, off by default, the words after pasting.
Position is one of six spots or free: drag the overlay, in the preview or on screen, and it stays where
you drop it with none of the six selected, until you pick a spot again. With several screens, choose the
one where you are typing, the one with the mouse pointer, the primary, or always one in particular (a drop
on another screen then makes that one the screen); if that one is unplugged, or plugged into another port, the overlay falls back to where you are typing.
A screen is recognised as the same monitor on the same connection. The style, colours and movement
follow your account, and the position and the screen stay on each PC.

- (overlay) Say when the text is pasted, and turn a refused dictation away with a shake

Once the text is in the window you were typing in, the overlay stays up for a moment with a tick
instead of vanishing the instant it is done, unless another dictation is still transcribing, which then
keeps the overlay. A paste that fails (a locked clipboard, a window that will not take it) never shows
the tick: the overlay says "Paste failed" and the refusal sound plays, even over a recording or another
transcription, which then gets the overlay back. When a dictation cannot start, the overlay appears in red,
shakes side to side like a head saying no, says why in a few words (no model, model still loading, no
microphone) and the refusal sound plays as before; a second refusal while the first is showing shakes
again. The words after a paste ("Pasted, 14 words") are off by default and the tick carries it. The
overlay is drawn on a stage with room round the pill, and the three arcs of the halo now turn on the
graphics card instead of repainting the border every frame, at the same speeds as before. Dictations
still transcribing behind a recording still show their count in the overlay, and cancelling a recording
in front of a queued dictation hands the overlay to it without drawing it in again.

- (overlay) Place the overlay on a screen's work area and keep it whole on the screen

The overlay now appears at the bottom centre of the screen where you are typing, above the taskbar,
instead of in the middle of the screen. With several screens it follows the window you dictate into,
and falls back to the one with the mouse pointer and then the primary screen. A position is kept as a
share of that screen's usable area, so a change of resolution or display scale no longer strands it
off screen, and on a screen at a different scale it keeps the same size. Drag it and it stays where you
drop it, wherever the screen rule then sends it. An overlay you had already moved keeps its place.
When the desktop itself is in front, it goes where the mouse pointer is.

- (window) Call the Transcription page Engine and the Preferences page Settings

The page that picks the model, the backend and the server is now named Engine (Moteur in French),
and Preferences is Settings (Paramètres). Engine comes first in the group at the bottom of the
sidebar, ahead of Appearance.

- (preferences) Give the account its own page

The account entry at the bottom of the sidebar opens an Account page instead of Settings. It shows
your avatar large, drawn from your address, with the provider, the address and when the last sync
ran, and the same actions as before: sign in (and cancel while it waits), sync now and sign out,
with a sync error written out where you can read it. Under it, a card lists what follows your
account from one PC to the other and what stays on each PC. The Account card is gone from Settings.

- (preferences) Give dictation its own page

Everything about how you dictate moved out of Settings onto a new Dictation page between Engine
and Appearance: the shortcuts, the recording mode, chained dictations, the feedback sounds, the
companion shortcuts and meeting mode. Settings keeps the audio devices, the system options, the
language and the updates.

- (preferences) Draw the window from a theme made of colours and a gradient

The Appearance page now opens on an Application tab with a grid of themes: eight gradient ones, the
classic editor palettes, and every theme the application had before. Picking one fades the window
over to it instead of switching in a blink. The accent is a gradient now, and the primary buttons,
switches, sliders, the highlight in the sidebar and the chosen cards take it; text on the accent
turns dark or light on its own, with a faint layer behind it when a gradient is too uneven for
either, so it stays readable. Text, status colours and the focus ring are held to a readable
contrast whatever colours you pick, even over the brightest lights (the cards turn more solid
when they have to), and the busiest days of the activity chart are always the
strongest. Cards frost against soft lights that drift behind them and stop when the window is
hidden or unfocused. The window opens in your theme from the first frame. Your old theme is carried
over to its match, and a change you make is kept with the rest of your settings. The recording
overlay settings sit on their own tab and are unchanged.

- (preferences) Edit the accent gradient, the lights, the base colours and the shape of the window

Under the themes, the Application tab lets you build your own look. The gradient has two to four
colours you drag along a bar, add by clicking the bar or with a button, and remove, an angle dial
that also answers the arrow keys, linear, radial or conic, a reverse, a random draw and a few
inspirations. The atmosphere sets the strength of the background lights, how much the cards let
them through, whether they follow the gradient or use three colours of their own, whether they
drift, and an optional grain. You can also pick the four base colours (light or dark follows the
background, and the text is kept readable), the corners, the text size and the speed of animations
(Reduced follows the Windows setting too, and stops everything but the spinners). A changed theme
is marked as modified with a way back, and Save as my theme keeps it in the grid and on your
account.

- (window) Put the window buttons on the left, in the top row of the sidebar

Shape and motion on the Appearance page has a new choice for the window buttons. On the left they
become three small dots next to the name in the sidebar, which show their symbols when you point at
them or tab to them; the strip across the top of the page stays as the part you drag. On the right is the same as
before. The choice follows your Google account.

- (window) Move the navigation into a wider sidebar and slim the title bar down to the window buttons

The sidebar now names its pages, shows the account you are signed in with at the bottom (it opens
the Account page), and collapses to icons with the thin handle on its right edge; it remembers which
way you left it. A highlight glides to the page you pick, and pages ease in when you switch,
unless your system is set to reduce motion. The strip along the top is only the minimize, maximize
and close buttons now. The title bar's model and server label is gone; when dictation is not
ready (no model, a model loading, server unreachable, token refused, or a fall back to the local
model) a small coloured pill says so above the settings pages in the sidebar, and a click opens
Engine.

- (transcription) Confirm before a change of backend or graphics card reloads the model

Switching between CPU and Vulkan, or to another graphics card, reloads the loaded model on the new
device, which can take a minute or more with a large model while dictation is unavailable. A dialog
now says so and names the model, with Switch and Cancel. It asks every time. Nothing is asked when
no model is loaded.

- (preferences) Be invited to sign in with Google, once

A strip under the title bar offers to sign in with Google to sync your settings and statistics
between your computers, and the setup wizard gets an optional last step that does the same, with a
Skip as prominent as the button. Either one is asked only once: signing in, skipping or closing it
settles the question for good, and neither shows when the build cannot sign in or when you are
already signed in.

- (preferences) Two more app themes: Catppuccin Mocha and GitHub Light

Appearance now offers six dark themes and three light ones, so each group fills its rows evenly.

- (dashboard) Switch the dashboard between all your devices and this one

Once another machine has synced, the dashboard shows an All devices / This device toggle. All
devices is what it showed before, the sum of every machine. This device counts only what was
dictated on this PC. It opens on All devices every time Talk starts, and nothing shows until a
second machine has synced.

- (preferences) Sign in with Google to keep several machines in step

A new Account page signs in with Google, which is entirely optional: nothing in Talk
needs it. Signed in, each machine keeps its settings, its statistics and its history in the app's
own hidden folder of your Google Drive, and shows the sum of all your machines: 98 hours saved on
one PC and 66 on another read 164 on both, and syncing again never counts anything twice. The
history page lists every machine's dictations by date. Settings follow the most recent change; the
server token, the audio devices, the graphics card and anything else tied to one machine stay
where they are. A sync runs at sign-in, at startup, on Sync now and every five minutes, and a
failed one only shows on the card. Reset stats now resets this machine's counts and removes its
uploaded statistics from Drive. Signing out keeps everything on this machine and leaves the Drive
files in place.

- (preferences) Settings follow you from one computer to the other, within seconds

Start with Windows, Start minimized and Meeting mode are now synced too. Meeting mode only changes where the virtual cable is installed, and the chosen model stays a
choice per computer. A change in Settings reaches your Google account about ten seconds later
instead of at the next five minute round, and bringing the
Talk window to the front syncs when the last round is over a minute old. Settings arriving from
another computer show up in the open window without restarting Talk.

- (recording) Chain dictations while one is still transcribing, and cancel what is transcribing

Recording again while a long dictation is still on its way through the model no longer risks the
texts landing out of order: they now come out in the order they were spoken. A new Chained
dictations card on the Dictation page decides whether each one is pasted as soon as it is ready or the
whole run is held and pasted once as a single paragraph, and whether the paste shortcut brings back
the last dictation or the whole last run.

  The cancel shortcut now also works once the recording is over. A first press still drops the
  recording in progress; with none left, it drops everything waiting, or only the one being
  transcribed, as chosen on the same card. On a graphics card the transcription itself can take a
  second or two to let go, but nothing it brings back is pasted.

- (overlay) Show what is still transcribing behind a new recording

Recording again while a dictation is still on its way used to hide it entirely. A small ring now
sits beside the timer, filling as the dictation in progress goes through the model, with the number
still waiting inside it. The crossed-out microphone meeting mode added to the overlay is gone:
meeting mode is a setting, and it read as a second microphone.

- (server) Use any OpenAI-compatible transcription server

Server mode no longer needs a Talk-Server. Point the URL at OpenAI, at
https://api.openai.com/v1 or without the /v1, or at any other server that offers
/v1/audio/transcriptions, and put its key in the field now called API key or token. A server
without the streaming route is detected on the first dictation and remembered until the app
restarts, and the overlay shows its state without segments. A new Model field picks the model
to ask for: leave it empty for a Talk server, and OpenAI falls back to whisper-1.

- (server) Find servers on the local network

A Talk-Server announces itself on the network, and the Engine page now lists the ones it
hears, with their name, model and address. Pressing Use fills the server URL, so there is nothing
to type. A server that goes away drops off the list by itself.

- (server) Offer a server found on the network once

When Talk is dictating on this machine and a server appears that it has not seen before, a strip
under the title bar offers to use it. Each server is offered a single time, whether you accept it
or close the strip.

- (server) Pair with a server instead of pasting its token

A server that offers pairing now shows a Pair button in the list on the Engine page, and
the token field has a Pair with this server link for an address you typed. Talk asks the server for
a code, you read the 6 digit code in the server's log or on its admin page and type it in, and the
token fills itself in. The connection is tested straight after, so the status reads Connected.

- (server) Share this PC with other machines

A new Share this PC card on the Engine page lets other computers dictate through the model
loaded here, with no Docker or Python on this side. Turn the switch on and Talk listens on port
8000, or the port you pick, and announces itself on the network, so another Talk running in Server
mode finds it in its list. That machine asks to pair, a strip under the title bar here shows a
6 digit code, and you read it out; each code is good for two minutes. Paired machines are listed on
the card and can be revoked, which cuts them off at once. Anything that speaks the OpenAI
transcription API can use it too, with the token pairing hands out, as long as it uploads WAV:
that is what Talk sends, and this PC refuses any other format. The first time the port opens,
Windows may ask whether to let Talk through its firewall, and other machines cannot connect until
that is allowed. Dictating on this PC always goes first: a request from another machine waits
until your dictation is done, is stopped if you start recording while it runs, and the other
machine is told to retry shortly.

- (preferences) Play any feedback sound again from its own button

Hearing the sound already picked meant choosing another one and coming back. Each sound on the
Dictation page now has a play button beside it, the refusal sound included.

- (preferences) Use the application in French as well as English

The whole interface, the tray menu included, now comes in English and French. It follows the
language of Windows by default, and anything that is not French shows in English. The Settings
page has a Language setting to pick one of the two whatever Windows is set to, and the change
applies at once, without a restart. Dates, times and numbers follow the same language.

- (server) Lay out the Engine page in the order it is used

In Server mode the servers found on the network now come first, then the connection, then a single
card holding the timeout and the fallback to this machine. In Local mode the Share this PC card
moved below the models, and while sharing is off it shows only its switch, plus the paired machines
if there still are some to revoke.

- Give every page and card the same look

Cards on the Engine, Dictation, Settings, Appearance and dashboard pages now share one header: an
icon in a small tinted tile and a bold title, with the switch or button that belongs to the card on the right and the
explanation under it. The server cards keep their blue icon and the model cards their cyan one.
Spacing inside cards and between them is the same everywhere. The save, cancel, edit, add, pair,
use, revoke, refresh and play buttons now share one size and style, and the fields share one focus
ring.

- Reorder and remove vocabulary terms by hand

Each term now has a six-dot grip on its left: drag it to put the terms in the order you want, and the
others slide aside. The cross is gone; hover the word and a line strikes it through, click it to
remove it. A new term pops in, typing one that is already there makes the existing one shake, and a
line under the list says how to reorder and remove. A press on a word that moves before you let go
does not count as a click, and a removed term can be put back with Undo, at its old place, for a few
seconds. Everything holds still if your system is set to reduce motion.

- (dashboard) Run the typing test in a dialog

The button that starts the typing test moved to the top right of the Time saved card, and the test
now opens in a dialog over the window instead of unfolding at the bottom of the page. It marks each
character right or wrong as you type and shows your words per minute, your accuracy and the time
as you go. Escape closes it and puts you back on the button, "Start over" gives you a new sentence,
and the result only replaces your typing speed when you keep it. Once the sentence is finished,
Escape and a click outside do nothing, so a result is never lost by accident: keep it or discard it.
The full-width button at the bottom of the dashboard is gone.

- (dashboard) Fit the dashboard to the width it is given

The dashboard now follows the width of its own page rather than the screen. When it is narrow, which
includes the default window, the period and device filters each take a full row, the four figures
and the four facts go two by two, and the three comparison cards stack. Labels in the facts strip
wrap between words instead of being cut off.

- (transcription) Word the Engine page as the design does

The switch reads "On this PC" and "On a server", the loaded model wears a Loaded pill and the models
card no longer counts what is downloaded, Download is a plain text button, and the card icons all
take the accent colour instead of green, amber and blue. A note under the graphics card says that
switching engine or card asks for confirmation while a model is loaded, sharing explains that it
needs no Docker or Python, the timeouts read "10 s" and "1 min", and "Pair with this server" sits
on the same line as the API key label.

- (recording) Word the Dictation page as the design does

Toggle mode says "One press to start, another to stop", the paste shortcut says "Wherever you are
typing", the cancel shortcut offers "Everything waiting" or "The oldest", and the refusal sound says it
plays when there is no model to dictate with. Meeting mode takes the microphone icon and says whether
VB-Cable is installed inside its own sentence instead of on a line with a dot.

- (preferences) Give the Account page its design

The glow behind the account card and the halo of the avatar follow your accent gradient, the
address is set tighter and the sync line a size smaller. Signed out, the card says "No account" with
a line inviting you to sign in, and the button carries the Google mark. "What follows the account"
lists short chips: shortcuts, vocabulary, sounds, theme and gradients, language, startup, dictation
preferences, volume and clipboard, statistics and history, and a second row says what stays on each PC.

- (window) Say a failed sync and a waiting update in the sidebar

The pill above the Settings link also appears when the last sync with Google Drive failed, and opens
the Account page, or when an update is waiting, and opens Settings. A problem with the engine still
comes first, then the sync, then the update. The account entry reads "Not signed in" and "Offline"
when you are signed out and "Sync failed" after a failed sync, and the fallback pill reads "Local
fallback".

- (dashboard) Date the typing test and shorten two titles

The "At the keyboard" line of the typing card says when the speed was measured, as from the next test
you keep (with the year when it is not this one), and the line below it says over how many dictations your voice speed was taken. The two
comparison cards are titled "Hosted APIs" and "Subscriptions".

- (history) Search the history

The History page opens on a search field instead of the "12 of 100 kept" line: typing narrows the
list to the dictations containing every word you typed, in any order and without regard to case or
accents, marks them, and a line says when none does; Escape clears the field. An
entry lifts a little under the pointer, and clicking it shows "Copied" with a check on its footer
line.

- (preferences) Keep the Add button bright on the Vocabulary page

The Add button no longer turns grey while the field is empty: it keeps its gradient and does nothing
when pressed, and assistive technology still announces it as unavailable. The field suggests
"e.g. MyProject, ACME", and an empty list says the first term you add goes to the model on the next
dictation.

- (preferences) Lighten the top of the Appearance page

The themes card opens straight on the previews without its introduction, "Save" is a quiet outlined
button next to a Reset appearance button of the same height, the gradient summary reads "linear,
135 degrees" without the colour count, the borders colour has no hint, and the shape card takes the
dashboard icon.

- (overlay) Order the overlay settings as the design does

In Movement, the timer comes first, then "Text at the end", then the microphone icon, and the
appearance setting says "When the recording starts". The style and movement cards take the
microphone and speaker icons, the position card the screen icon, the capsule is described as a black
pill, the orb as smiling when it is pasted, and the preview's fake window is titled "Notes, product
review".

- (preferences) Tidy the Settings page

The audio devices no longer count how many were found, the hints under "Start minimised" and "Turn
the volume down" are shorter, and the updates card has its check button in the header, the refresh
icon, an Install button beside it when an update is waiting, and one note under it that gives the
version and where the check stands.

- (overlay) Draw the orb a little smaller

The orb is a touch smaller on screen and in the preview, with the avatar in proportion to its
ripples, and it stays in the middle of its window.

- (preferences) Set the sign-in page in Geist

The page the browser shows when you come back from signing in with Google uses the same typeface as
the application.

### Bug Fixes

- (recording) Every way a dictation ends now tidies up the same things

A dictation that ended some other way than a normal stop used to forget part of the clean-up. A
paragraph held for a recording whose microphone then failed to open was never pasted, a handler
error could leave the overlay on "recording", the virtual microphone muted and the volume down, and
a cancelled text could still be pasted by the paste-last shortcut. All of them now put everything
back, and the sync, if it was waiting, runs. The waveform of a recording also stops with it, even
when you start another one right away.

- (preferences) Leave a dictation alone while the account syncs

Settings coming from your account used to be applied whenever the periodic sync happened to run,
which could be in the middle of a dictation: the shortcuts were registered again and the overlay was
moved under you. The sync now waits until the recording, the transcriptions and the pasting are
over. A "Sync now" pressed meanwhile says on the Account page that the settings will be applied when
the dictation ends, and they are.

- (audio) Say it when the microphone is unplugged during a dictation

A microphone pulled out, or a headset that dropped, in the middle of a dictation used to leave the
recording going on as silence, with nothing to tell you. When you release the shortcut, what was said
before the microphone gave out is transcribed and pasted as usual, and the refusal sound and
"Microphone lost" on the overlay then tell you it stopped. A dictation with nothing captured before
the failure is turned away the same way, and nothing is pasted.

- (recording) Cancel works while a text is being pasted

Pasting a dictation takes most of a second, and for all of it a cancel, or the next dictation
finishing, had to wait. Only the pasting itself is now held in line: texts still come out in the
order they were spoken, and never run into each other. A text that is done and waiting for its turn
to be pasted is cancelled like one still being transcribed, and a paste already typing finishes.

- (models) A dictation made while a model loads waits for it

Dictating while a model was being loaded, or reloaded for another graphics card, could lose the
text without a word. The dictation now waits for the model to be there. Loading two models in a row
no longer builds both at once.

- (overlay) Show the overlay as soon as the shortcut is pressed

The overlay used to appear only once the microphone was open, which on some devices takes a moment
during which nothing answered the key. It now comes up first, and turns to "No microphone" if the
microphone cannot be opened. A failed start also gives the virtual microphone back its sound.

- (recording) A quick tap no longer leaves the microphone open

Pressing and releasing the push to talk shortcut very quickly could stop the release from being
noticed, and the microphone then stayed open until the next release. Presses and releases are now
handled one after the other, in the order they happened, so a tap starts and stops the recording
cleanly.

- (preferences) Never save a half-written settings file

A crash or a power cut while the settings were being saved could leave a half-written file, and the
next change then saved the defaults over your server address, your token and your shortcuts. The
file is now written whole or not at all, and waits a moment when another program, a virus scanner
or a backup, has it open.

- (preferences) Keep the original of a settings or shortcuts file that cannot be fully read

If the settings or the shortcuts file does not parse, or holds a value this version cannot read, the
original is kept next to it as settings.unreadable.json or hotkeys.unreadable.json (never replaced
by an emptier copy), the file is rewritten at once from what could be read so that the next start is
clean, and a message tells you so once, at the first start you can see and not over an autostart at
login, in the language of the interface. One value that does not fit, however deep in the file, no
longer sends the whole file back to the defaults: only that value falls back. A file saved with a
byte order mark, as Notepad and PowerShell used to, is read as the intact file it is.

- (preferences) Say so when the settings file cannot be opened

If another program, a virus scanner or a backup, holds the settings or the shortcuts file for more
than a few seconds when Talk starts, Talk now says which file and closes, instead of running on the
defaults and saving over it. It does the same when the original of a file it cannot fully read could
not be kept first, and says which of the two went wrong.

- (preferences) Do not sync the settings in a session that started on a file it could not fully read

Such a session neither pushes its defaults to your account nor takes your account's settings over what
you changed meanwhile, and the Account page says so. The next start merges the two the way a first
sign-in does.

- (preferences) Never lose a change made at the same moment as another

Two changes landing together, such as a switch you flip while the settings sync with your account,
could overwrite each other and one of them was lost. Every change now waits for the one before it
to be written. A setting that could not be saved is no longer applied, and the control that asked
for it now gets the error.

- (models) A failed model download no longer counts as downloaded

When the model server answered with an error, its message was saved as the model and listed as
downloaded, then failed to load. The download now stops with a readable error and leaves nothing
behind. Asking for a model that is already downloading is refused, a download that stalls ends in
an error instead of blocking that model until you restart, cancelling takes effect at once, and
partial files left by an earlier crash are removed at launch.

- (window) Say why the app closes when its database will not open

When the history database could not be opened at launch, the app vanished without a word. It now
shows a message with the file it tried and the reason, then closes.

- (recording) Quit from the tray puts your volume back

Quitting from the tray icon while a dictation had turned the volume down left it down. It now goes
back to where it was before the app closes.

- (window) Stop a page flashing before it slides in

When you opened a page it could be painted in its final place for a frame, then jump back and slide
in. The page now starts from its faded pose before anything is drawn.

- (preferences) Never lose your vocabulary to a sync

Signing in on a second computer replaced its vocabulary with the account's, so a list built by hand
could end up empty. Vocabulary now merges between computers: a word added on either one reaches
the other, and a word you remove disappears from both at the next sync. The first sign-in on a
computer also keeps the settings you already chose there, and only fills in the ones still at their
default from the account. Shortcuts for companion apps are combined the same way.

- (preferences) Show a readable sync error on the Account page

A failed sync used to print Google's whole answer, thirty lines of it. It now shows the message
Google gave, cut to a sentence, on at most three lines with the full text on hover, and an address
in it opens in your browser.

- (dashboard) Show the full labels on the dashboard cards

Labels and sub-lines on the four figures at the top and on the hosted API and subscription cards
were cut with an ellipsis at the default window width, and the typing speed row wrapped onto three
lines. They now wrap instead.

- (preferences) A proper page after Google sign-in

The browser tab that opens when you sign in with Google was a bare line of black text on white. It is
now a small card in Talk's typeface and colours, light or dark following the system, in English or
French following the browser, with a check mark, and a clear message when Google refuses. It says
the tab can be closed, and Talk comes back to the front on its own.

- (history) Stop the history cards turning grey, or black, under a quick mouse

In a light theme, sweeping the mouse across the history left cards grey after it had moved on, and
sometimes flashed them almost black. The highlight now simply follows the pointer.

- (preferences) Cancel a Google sign-in that went nowhere

When Google refuses a sign-in on its own page and never sends you back, Talk used to sit on
Waiting for the browser with every button disabled until the wait ran out. A Cancel button now
sits next to it on the Account page, on the strip under the title bar and in the setup wizard, and
puts things back as they were without an error. The wait itself now gives up after three minutes
instead of five.

- (installer) Keep your data in a folder named after the product

Your settings, history and downloaded models used to live in a folder still called t4lk. It is now
named Talk, and the first start after updating moves everything there by itself: there is nothing
to do, and nothing is deleted. If the move cannot happen right then, Talk keeps using the old
folder and tries again at the next start. Deleting the application data at uninstall clears both
folders.

- (preferences) Line up the shortcut cards, and say what Cancel throws away now

The keys of the main shortcut and of Cancel sat at different heights whenever one description ran
to two lines; they now both sit at the foot of their card. Cancel's description still said it only
stopped the recording, when it also throws away dictations not pasted yet.

- (preferences) Keep the Updates card at the bottom of Settings

Cards were added to the page in the order they were written, so Companion shortcuts had landed
below Updates. Updates is now the last card, whatever comes next.

- (server) Send the server token, so server mode transcribes on the server again

The token typed on the Engine page was saved and never sent. The server refuses every
transcription without one, so each dictation in server mode was turned away and quietly done on
this machine instead, while the connection indicator, which asks a route that needs no token, kept
showing the server as reachable.

- (server) Tell a refused token from a server that does not answer

The connection test used to ask a route that needs no token, so a wrong token still showed the
server as connected. It now asks one that does. A server that is there but turns the token away
says so on the Engine page, the dashboard and the title bar, and "Server unreachable" is
kept for a server that does not answer. A server too old to check the token on still shows as
connected, as it always did.

- (recording) Say so when there is no model, instead of recording for nothing

In local mode with no model loaded, the shortcut used to start a recording as usual and the text
simply never came, which looked exactly like a microphone that heard nothing. It now plays a short
low double note and the overlay says "No model loaded", or "Model still loading" in the seconds
after launch, and nothing is recorded. The main window also shows a strip saying no model is
loaded, with a button to the Engine page, for as long as that stays true.

- (installer) Keep the downloaded models when installing a newer version

Installing a new version over an older one runs the older uninstaller first, and that uninstaller
deleted the models every time, so the first dictation after an upgrade had to download a gigabyte
or more again. Models, settings and history are now only removed when "Delete the application
data" is ticked while uninstalling.

- (vocabulary) Keep the terms their own size while one is dragged, and give the list the page

Dragging a term to reorder it stretched or squeezed the ones it passed over to the width of the
term being dragged. They now only slide. The list of terms also used to stop after a few rows with
a scroll of its own in the middle of the page; it now grows with the terms it holds, and only
scrolls once it reaches the bottom of the window, while the field to add terms stays in view.

- (window) Say that the model is loading while the application starts

The sidebar pill that reads Loading model only ever appeared when you loaded a model by hand. At
launch, while the last model was coming back, it stayed empty. It now shows for that wait too.

- (window) Stop the options that slide open from moving when your system reduces motion

The extra options under feedback sounds, companion shortcuts and the system settings slid in even
with reduced motion on. They now appear at once.

- (window) Name the switches, lists and icon buttons for screen readers

Every switch and drop-down on the Engine, Dictation and Settings pages, and the buttons that only
carry an icon (check the connection, delete a model), now announce what they are for.

- (dashboard) Keep typing after Start over in the typing test

When Start over drew the same sentence again, the focus stayed on the button and the keys went
nowhere until you clicked the sentence. The field has the focus back every time.

- (preferences) Say so when a setting could not be saved

A switch, a choice or a field that the application failed to save used to keep showing the new
value as if it had worked. It now goes back to the last value that was really saved, and a message
at the foot of the content says the change was not kept, once, however many steps of a slider it
took. Saves of one setting go one at a time, so two quick changes can no longer undo each other.
The server address, token and model fields and the theme go back to what was saved too. When
another PC's settings arrive while you are changing one, the screen never shows a value that
neither you nor the application holds, and a refused save goes back to the new value.

- (window) Keep Escape for what is using it

The message at the foot of the window no longer disappears when you press Escape to close a
rename, a menu, a field or a dialog: only an Escape that nothing else wanted dismisses it.

- (history) Keep deleting and clearing true

Deleting a history entry or clearing the history that fails shows the list as the application
holds it, whatever was dictated or deleted in the meantime: a deletion never leaves a row that a
clear removed, and a refused clear keeps your search and the page you had opened. A deletion
issued before a clear is carried out before it.

- (window) Say so when the application cannot start

If the application could not tell whether setup was done, the window stayed on "Loading" for ever.
It now says so and offers to try again, and the window can still be moved and closed. And when one
of the first reads of your settings or history fails, the others still load, a message tells you
part of it could not be read, and what depended on it is locked, with a Try again beside it, so
that nothing can save a default over what is stored. Try again also reads again the models and the
graphics cards that could not be read, and loads the last model once they have come back.

- (preferences) Remove vocabulary terms quickly and from the keyboard

Removing two terms in a row no longer puts the first one back, and a term that could not be saved
comes back with a message. Removing a term with the keyboard moves the focus to Undo instead of
dropping it, and when Undo goes away the focus lands on the next term. After a refused edit the
list shown is the one the application holds, and an edit that a later one carried counts as done:
Undo is offered and the field is emptied.

- (preferences) Say so when a device, sharing or overlay setting fails

The chained dictations choices and the microphone and speaker pickers go back to their previous
value when the save fails. Turning sharing on or off, revoking a paired machine and editing the
overlay now show a message instead of failing silently, and the overlay settings show what was
really kept. These choices, the language and the meeting mode switch stay locked until they have
been read, and say so with Try again when they cannot be; the overlay tab no longer stays blank
when its settings cannot be read. They follow the settings another PC brings.

- (dashboard) Never show 100% local beside dictations through the server

The share of dictations kept on this PC is no longer rounded up to 100 (or down to 0) while some
dictations went the other way.

- (preferences) Keep the shortcuts working after editing one

Leaving the Dictation page in the middle of editing a shortcut used to leave every dictation
shortcut off until you restarted the application. They now come back when the edit ends, whether
you click away or leave the page, and a refused save keeps them off while you try again.

- (preferences) Let the keyboard leave a shortcut field

While a shortcut field waited for keys, Tab and Escape were captured like any other key and the
focus was stuck in it. Escape now gives the capture up and Tab moves on to the next control. In the
shortcut editor, Escape, Cancel and a successful Save put the focus back on that shortcut's Edit
button.

- (dashboard) Do not keep a pasted typing test

Pasting or dropping the sentence into the typing test counted as typing it in a millisecond and
produced an absurd speed. Pasting is refused, with a line saying why, and a result no one can type
(300 words a minute or more) is flagged and cannot be kept.

- (dashboard) Keep the focus inside the typing test

Clicking the edge of the typing test no longer leaves you typing into nothing, and Tab no longer
reaches the page behind it. A result that cannot be kept leaves the focus on Start over, and
Escape discards it.

- (preferences) Name the buttons that had no name

The handle that reorders a companion shortcut now announces what it does, and the close button of
the pairing panel reads "Close" instead of a raw key.

- (history) Respect reduced motion in the history and its dialogs

With motion reduced, the history rows, the "Copied" badge and the confirmation dialogs no longer
spring, scale or slide: they appear and go at once.

- (history) Keep a long history light

The history shows its first fifty dictations and loads fifty more on request, so a history of
hundreds no longer weighs on every refresh of the page. Search still looks through all of it, and
showing more moves the focus to the first new dictation. Only a dictation that arrives on its own
is animated.

- (overlay) Write the reaction percentage in the interface language

The reaction slider of the overlay used to print its percentage the same way in every language. It
now follows the language's own spacing, like the dashboard.

- (dashboard) Print a whole number of hours as "98 h"

The time saved read "98 h 00" on the Account page and the dashboard when the minutes were zero. It
now reads "98 h".

### Performance

- (window) Hold the avatars still on a machine that cannot keep up

On a computer without a graphics card, the animated avatar of the sidebar and of the Account page
kept most of a processor core busy for as long as the window was open, and the Account page could
become slow to answer. The speed of the window is now looked at every few seconds instead of only at
launch, and the avatars hold still, like the lights behind the window, when it is too slow or when
the window is not the one in use.

- (recording) Start dictating without waiting on the disk

With the volume lowering switched on, every dictation rewrote the settings file and ran the sync's
bookkeeping before the overlay appeared. The overlay now shows without touching a file. The level
to restore is kept in memory, with a small marker file of its own that only a crash leaves behind,
so the volume still comes back at the next launch, including after an update from a version that
died while the volume was lowered.

- (audio) Keep a very long recording smooth

Once a recording went past ten minutes, every slice of audio made the app move the whole recording
around, which could make it stutter. It still keeps the last ten minutes, and now does so without
the cost growing with the length.

- (preferences) Never leave an empty file on your Drive

A new file was created empty and filled in a second request, so a dropped connection or a closed
app in between left an empty file that the other computers then choked on. A file now reaches Drive
in one request, with its content.

- (preferences) Keep the account's startup settings after the setup

Signing in with Google during the setup applied the account's Start with Windows and Start minimized
choices, then finishing the setup put the wizard's own switches over them and sent those up to the
account. The two switches now start from the account's values once you have signed in, and
finishing the setup writes only a switch you moved yourself.

- (preferences) Carry on past a damaged file on your Drive

A settings file left empty on the account blocked the settings sync on every computer until somebody
deleted it by hand. A file Drive lists as empty is now filled in by the computer that finds it, and the
same goes for the device list. A file that has content but cannot be read is never written over: it
may come from a newer Talk, so the settings or device sync is skipped for that round and retried at
the next one, while statistics and history carry on. An unreadable statistics or history file from
another computer is left out, keeps what was held for that computer, and does not stop the rest of
the sync.

- (preferences) Say in plain words why a sync or a sign-in failed

The Account page showed Google's own text, or a raw network error. It now says in your language
whether Google Drive could not be reached, whether Talk's access to the account was revoked or has
expired, whether the Drive API is switched off or refused, whether your Drive is full, whether the
account's settings or list of devices come from a newer Talk, or whether Google is limiting requests.
Google's own text stays under the wording when it adds something. An expired access token is renewed
once before anything is reported. A sign-in that Google cannot name an account for is refused before
anything of the connected account is touched. When another computer's statistics or history cannot be
read, the sync still succeeds and the page says so under the last sync time. When the access is gone the page offers Reconnect, which signs in again without
signing you out and keeps what the account already synced. Reconnecting with a different Google
account does what signing out and in again does.

- (preferences) Show how long ago the last sync was

The Account page gave only the hour of the last sync, so a sync from last week looked like one from
this morning. When it is not from today the page now says how long ago it was.

- (preferences) List statistics and history among what follows your account

The Account page listed only settings under What follows your account, although statistics and
history are synced between your computers too. Both are in the list now.

- (window) Change pages without a blur

Pages still fade out and fade in, but they no longer blur while they move, which is lighter to draw
on a machine without a graphics card.

- (window) Keep a very large window smooth

The coloured lights behind the window stop growing past a certain size, so maximising the window on
a large screen no longer makes it slower to draw than a medium one.

- (preferences) Make a theme change lighter

Switching theme still fades the surfaces, the text, the status colours, the accent and the lights,
while the gradient stops and the focus ring take their new value at once.

- (window) Check the lights again when the window changes size

If the window is made much larger or much smaller some time after launch, the speed of the lights is
measured again: they hold still when the new size is too slow to draw them, and move again when it is not.

- (server) Stop checking the server while the window is not visible

In server mode the connection is no longer tested every few seconds while the window is not visible
to the page, and it is tested once as soon as the window is visible again.

- (models) Keep the window light during a model download

The progress of a download no longer redraws the whole window several times a second, only the
bar that shows it, and coming back to the Engine page in the middle of a download shows the bar
where it was.

## [0.9.0] - 2026-09-07

### Features

- (history) Paste the last transcription without coming back to the window

A dictation that landed in the wrong window meant opening Talk, finding the card at the top of the
history and copying it. Ctrl+Shift+Space now puts that text wherever the caret is, and the
combination is changed on the Preferences page like the other two.

  It answers with what the history shows, so it still works after a restart, and it waits for the
  keys of the shortcut itself to come back up before pasting: pressed while Ctrl and Shift are
  still down, the window in front would read Ctrl+Shift+V, which is a different command in most
  editors and browsers.

## [0.8.1] - 2026-08-29

### Bug Fixes

- (overlay) Draw the overlay in front, and not behind the window in the way

Windows takes a window out of the always-on-top band on its own account, and the overlay had no
way back into it: a dictation then drew it underneath the browser, the editor or Talk's own
window, so the shortcut played its sound and nothing appeared, and restarting the application was
the only thing that repaired it. Every dictation now puts it back in front first.

## [0.8.0] - 2026-08-27

### Bug Fixes

- (window) Give the sides back, so the page really gets 785 by 845

The same invisible frame that took nine pixels of height takes sixteen of width, measured on the running window: a configuration of 785 left the page 769 across. Both numbers now describe the frame rather than the page.
- (window) Count the frame, so the page really gets 785 by 845

Measured on the running window: the configured size lands on the outer rectangle, and Windows takes nine pixels of invisible resize border out of the height before the page sees any of it. A window asked for at 845 gave the home page 836, which is exactly enough to put a scrollbar on it.

  Nine go back on, to the size it opens at and to the smallest it can be made. The width was already right: the sixteen pixels the frame takes there are added outside rather than removed inside.
- (transcription) Keep the backend tile still while the card is switching

Changing card reloads the model, and the reload was driving the same loading flag as a change of backend, so the Vulkan tile went back to its spinner and validated itself again over a decision nobody had touched.

  The spinner belongs to the card being switched to. Both backend tiles go quiet for the length of the reload instead, which is also what stops a backend change from being started in the middle of one.
- (recording) Do not record a dictation that came back empty

Whisper answers with an empty string for a recording that carries no speech, and a server can answer with nothing in it while still answering. The save path ran on whatever came back, so such a recording was pasted as nothing, saved as a blank card, and counted.

  The count is the half that cannot be repaired afterwards: the history prunes itself and the blank cards leave with it, but daily_stats is permanent, so the total and the local against server split keep a dictation that never happened.
- (sound) Fade both ends of a feedback sound

The presets only ever had a decay, which never reaches silence, and no attack at all. A
  wave that opens at full amplitude, which the square one does by construction, puts a step
  in the signal, and a buffer that stops mid-cycle puts another one at the end. A speaker
  reproduces each as a click, and the click is what made a hundred millisecond beep sound
  mechanical rather than played.

  Both ends are shaped now, on a raised cosine rather than a straight line, since the
  corner where a linear ramp meets the note is itself audible on a sound this short. The
  attack stays under twelve milliseconds so the beep still lands the instant the recording
  starts, the release runs longer because nobody is waiting on that end, and both are taken
  as a share of a short sound so the fifty millisecond click is shaped rather than
  swallowed.
- (recording) Slide the volume down instead of cutting it

One call to SetMasterVolumeLevelScalar moves the whole machine in a single sample, which
  is heard as a cut rather than as the room being turned down, and it lands right at the
  moment somebody starts speaking. The move is now a slide: twelve steps, quicker on the
  way down than on the way back, since getting out of the speaker's way is the urgent half
  and coming back only has to sound natural.

  Both slides run on their own thread. The start path still has an overlay to show and an
  event to emit, and the stop path answers a shortcut, so neither can wait a third of a
  second on COM calls.

  Two slides can be in flight at once, and the second half of this is what keeps them from
  fighting. Each takes a ticket and stops as soon as a newer one exists, so the last order
  given wins rather than the last one to finish. A recording that starts while the volume
  is still coming up leaves the stored level alone instead of writing down a level read
  halfway through a slide, and the restore that was interrupted no longer clears it: the
  newer one does, once the volume is actually back.
- (history) Drop from the list on screen what the database just pruned

Rust prunes as it saves, the page did not, so the list grew past the limit for as long
  as the window stayed open. After four dictations on a limit of a hundred the header read
  "104 of 100 kept", and the four oldest rows on screen had already been deleted: clicking
  one deleted nothing, and a restart made them disappear with no explanation.

  The listener is registered once, so the limit comes through a ref rather than the closure
  it was captured in, the way the companion shortcuts already do it. Zero still means keep
  everything.
- (transcription) Do not keep a model that no longer loads, nor a card that never took it

A review of the two previous commits found three ways the state could lie, all on the
  failure path nobody walks until a driver misbehaves.

  A reload that fails left the engine empty while current_model still named a model. Every
  caller reads that as a model that is loaded, so the next dictation answered nothing at
  all instead of saying what had happened. The model is forgotten with the engine now, and
  the pages that changed the card ask the backend what is loaded rather than assuming.

  set_gpu_device wrote the choice to the settings file before knowing whether the card
  could take the model, while the interface rolled its own selection back when the call
  errored. The two then disagreed until the next restart, which read the file and picked
  the card that had just failed. The choice is held in memory for the reload, written to
  disk only once it worked, and put back where it was otherwise.

  Changing the backend had no rollback at all while changing the card did, three lines
  apart. Both roll back now.

  The sound worker also dropped its open stream whenever the device enumeration answered
  nothing, which happens for a moment while Windows moves devices around, losing a beep
  that the stream already open would have played. A hiccup leaves the stream alone.

  The README claimed the card with the most memory wins. It is the discrete card that
  wins, which is the whole point, since an integrated chip reports the shared system
  memory as its own. It also still described the recording as pausing what plays, which
  stopped being true when ducking replaced the pause.
- (recording) Duck by a share of the volume, not down to a fixed one

The percentage was read against full scale, so it was a floor rather than an
  attenuation: nothing happened at all when the machine already played below it. That
  is not a corner, it is where most people sit. Measured on the machine that reported
  it, the volume was at 26 percent of the scale against a setting of 30, so the guard
  that skips a machine already quieter than the target fired on every recording and the
  sound never moved once.

  The figure is now a share of the level found when the recording starts: at 30 percent,
  a machine at 26 goes to 8 and comes back to 26. The guard stays and only fires where
  it means something, a setting of 100 or a machine already silent. The slider reads
  "30% of it" rather than a level, since that is what it does now.

  Nothing to migrate: the stored numbers keep their range and their spirit, they are
  just read against what is playing.

  The comment punctuation in hotkeys/mod.rs comes back to ASCII on the way through.
- (sound) Follow the output device instead of the one that was default at startup

The stream was opened once, on whatever Windows called the default at launch, and a
  thread parked forever kept it alive. Plugging a headset moves the default and leaves
  that stream where it was, so the sounds went on playing to the speakers, and unplugging
  the device it held left them nowhere at all. Nothing in cpal follows the default for
  you: the only way is to open a new stream.

  The worker thread owns the stream now rather than handing a handle out, and names the
  device it should be on before each sound. When the name has moved, the stream is opened
  again on the new one. It costs a call per sound, and it is the only moment cheap enough
  to notice a headset arriving without polling for it.

  A device can also be pinned, the way the microphone already is. Preferences names it
  under the sounds it governs, with the system default first and the current default
  spelled out beside it. A pinned device that is absent falls back to the default instead
  of playing to nothing, so unplugging the headset it names is not a silent app.

  get_default_input_device lands with it: the microphone section has been calling that
  command since it was written, and swallowing the error, which is why the default was
  never named there either.
- (history) Hide the retention behind the count it governs

A labelled dropdown beside the title spent a control on a setting that is read
  far more often than it is changed. The line under the title already names the
  number, so the caret sits at the end of it and the line reads "128 of 500 kept".

  Everything comes off the menu, and the range stops at 500. Zero is still
  honoured everywhere that reads the setting, including the Rust that prunes: a
  settings file written while the option existed has to keep working.
- (history) Say "Keep" beside the dropdown, not inside every option

Repeating "Keep 50 transcriptions" on each line made the list long enough to
  read as a sentence and hid the number, which is the only part that differs. The
  label sits outside now and the options are the quantities.
- (dashboard) Stop refetching in step with the write it depends on

The dashboard listened to transcription-complete, which is the same event that
  triggers the insert: both handlers ran on it, and the analytics query went out
  alongside a fire-and-forget db_add_transcription. It usually read the database
  as it was before, so the figures sat one dictation behind, every time.

  The write now says when it has landed, and the dashboard answers that instead.

  Changing the retention warns first when it would delete, which it did not. The
  decision moves into lib/retention.ts so it can be tested without driving a Radix
  select through jsdom, and the warning only appears when something actually goes:
  a dialog that fires with no consequence teaches the reader to dismiss it unread.

  The three confirmations share one ConfirmDialog. Two had already drifted apart,
  which is what made the reset control feel unlike the clear one, and a third copy
  would have drifted further.
- (dashboard) Put the bin back on the reset control

The reset arrow was a change nobody asked for. The bin matches the history
  page, which is the point of the alignment.
- (dashboard) Ask before resetting the stats the way the history does

Two destructive controls, two different behaviours: the history opened a modal
  with a backdrop and an Escape, the dashboard swapped its own button for a Cancel
  and a Confirm under the reader's cursor. Wiping the counts is the same kind of
  act and now asks the same way.

  The bin becomes a reset arrow, since nothing is deleted here: the counts go back
  to zero and the transcriptions stay in the history, which the modal now says.

  Icon only, with an aria-label and a title. An icon button with no accessible
  name announces nothing at all to a screen reader.
- (dashboard) Drop the strikethrough on the compared amounts

Thinning the line kept the digits readable but did not make the effect worth
  having. The colour already says the money would have gone out, and the zero
  underneath says it did not.
- (recording) Let a dictation start while the previous one transcribes

Starting a second dictation already worked: is_recording is cleared as soon as
  the audio is taken, well before the transcription runs. What did not work was
  everything around it.

  The overlay is one shared window, and the first transcription to finish hid it,
  pulling it out from under whatever had started since. It is now held by a lease
  that releases on drop, so the last one out turns the light off and every early
  return is covered, which the old code path was not.

  The local engine ran on a runtime worker while holding a blocking mutex for the
  whole of a synchronous transcription. A second dictation stopping in the
  meantime had nowhere to run. It moves to the blocking pool, where work like that
  belongs.

  Both local call sites go through one helper now. They had drifted into two
  copies of the same block.
- (dashboard) Keep the struck amounts readable

The comparison figures were struck with a 2 pixel line in the same red as the
  digits, which on text that small closed the counters and buried the number the
  card exists to show.

  The strike stays, since it is what says the amount was never paid, but it drops
  to a hairline at 40 percent so it reads as a strike rather than as a bar across
  the glyphs.
- (vocabulary) Dedupe within a single input as well

The check compared each word against the existing vocabulary only, so pasting
  "tauri tauri" in one go added it twice. Comparing against everything accepted so
  far, the existing terms included, closes it.

  The parsing moves to src/lib/vocabulary.ts to be testable on its own. The view
  keeps calling it in the same place.
- (overlay) Default to the small size

Rust opened a new installation at medium while the frontend showed small
  selected and fell back to it, so the checked box disagreed with the window on
  screen. Small is the size the preferences were built around, so Rust follows.

  Existing installations keep the size they already wrote.
- (preferences) Start the sound state on the Rust defaults

The frontend opened with feedback off and both sounds set to none, then
  corrected itself once get_sound_feedback and friends answered. Preferences
  flashed the wrong state on every visit, and kept it whenever one of those
  calls failed, since the catch arms fell back to off as well.

  Rust has always defaulted to feedback on and beep on both ends. The initial
  state and the catch arms now say the same thing.
- (overlay) Default to the Frost theme

Aurora was the default a new install landed on. Frost is the one that reads
  well against the widest range of backgrounds, so it is the better first
  impression. The frontend fallback follows, otherwise a settings file with no
  theme written yet shows one theme while the Rust side uses another.

  Existing installations keep whatever they already chose.

### Build

- Run cargo from src-tauri so its config is read

cargo discovers .cargo/config.toml from the working directory upwards and pays
  no attention to where --manifest-path points. The config lives in src-tauri, so
  calling cargo from the repository root meant CC, CXX and CMAKE_GENERATOR were
  never applied: cmake fell back to the Visual Studio generator, which dies in the
  deep TryCompile paths under vulkan-shaders-gen.

  Nobody hit it because whisper-rs-sys was already built in the shared target
  directory. A fresh clone or a fresh worktree forces the rebuild and finds it.

### Documentation

- (repo) Keep main as the default branch, and say what that costs

It is what the repository page shows, what a clone lands on, and where the releases hang. A new pull request therefore opens against main unless it is told otherwise, so naming dev as the base is part of opening one.
- (repo) Say that a pull request title is a Conventional Commit too

It is what the repository shows for that branch forever, and what a squash would write into the history. A branch spanning several scopes takes the type of what it mainly delivers and drops the scope.

  Branches come back by rebase for the same reason the messages are written carefully: squashing would leave one line for a batch and throw the reasons away.

### Features

- (overlay) Scale what is inside, not just the window around it

Choosing a size moved the window and nothing else: every glyph, bar and spacing kept its own size, so the room around them grew and medium against large was a difference nobody could see from a step away. The point of the setting is a reader who cannot see the small one.

  The overlay is drawn at one size and scaled to the window, by the smaller of the two ratios so the pill never runs past the edge. Large goes to 341 by 93, a real step rather than a nudge, and the sizes keep the shape the drawing assumes, which a test now holds them to.
- (updater) Take new versions from the releases page on its own

The installed application polls latest.json, published as an asset of the
  newest release, and offers what it finds in a strip under the titlebar. It
  looks ten seconds after launch and once an hour after, since a window here
  stays open for days.

  Nothing installs unless it carries a signature made with the private half of
  the key in tauri.conf.json, which is why the release workflow now needs the
  signing secrets and why a build without them stops rather than shipping an
  installer no client would accept.
- (models) Stop a download, and ask before deleting a model

A model is around a gigabyte. Nothing was watching for a change of mind once a download had started, so a wrong click on a slow line held the page for a quarter of an hour with no way out but quitting the application. A cross beside the bar raises a flag the download reads at its next chunk, and the partial file goes with it.

  Deleting went straight through, which is the same gigabyte to fetch again over the same line. It now asks, through the dialog the history and the statistics already share, and says what it costs.
- (window) Open at 785x845, and never smaller

The size it opens at is also the size it stops at: below that the pages start folding, and there is nothing to gain from letting a window get there. Narrow and tall suits pages that are lists.
- (window) Open at 1140x825

Narrower and taller. The old size was set so the home page fitted in one screenful with the typing test at the foot; that page has gained a row of facts since, and the pages beside it are lists that read better tall than wide.
- (dashboard) Measure what the top row shows, instead of repeating the cards below

Three of the four figures at the top were the headline of a card further down: not spent
  is the hosted API card, time won is the time card, and the word count is a line inside
  that same card. The row said what the reader was about to read anyway.

  What replaces them was already being recorded and never read. Every dictation saves its
  audio duration and its processing time, so the speaking rate comes off the audio rather
  than off the fixed 150 words a minute the estimate divides by, and the real time factor
  says whether the card the engine runs on is earning its keep. Both are sums over the
  dictations still kept, which the history limit prunes, so the count they rest on is shown
  beside them and a fresh install reads "not measured yet" rather than an infinity.

  A strip under the row carries what the database knew and nobody displayed: how long you
  have been dictating, the streak, the busiest day, and the split between local and server,
  which Rust has always computed and the interface threw away. The streak is deliberately
  counted over everything rather than over the selected window, and a day still open does
  not break it.

  The comparison cards lose their Talk row. Zero against a hosted API is not news, and the
  line took the eye away from the prices it sits under. Where a dictation went is worth a
  word, though, so the API card now says how many went through the server, which costs
  whatever that server costs.
- (window) One instance, and nothing on screen when it starts in the tray

Two things the launch got wrong.

  The window was declared visible, so Windows painted a white rectangle for as long as the
  webview took to render, about half a second, and starting minimised meant hiding that
  rectangle rather than never showing it. The window is built hidden now and the page asks
  for it once React has painted something into it. A launch meant for the tray owes no
  appearance, so that request is swallowed and nothing appears at all. A net behind it
  shows the window after five seconds if the page never asks, since a frontend that fails
  to load would otherwise leave the application running with only a tray icon.

  The desktop shortcut opened a second application every time, each with its own window,
  its own tray icon and its own claim on the global shortcut. tauri-plugin-single-instance
  turns the second launch away and brings the running window forward instead, which is
  what clicking the shortcut is asking for. It sits first in the builder chain, as the
  plugin requires. A second launch carrying --minimized, which is what autostart passes,
  is left alone rather than being turned into a window nobody asked to see.
- (transcription) Pick the card the local engine runs on

Whisper took whatever the driver listed first. On a laptop carrying an
  integrated chip beside a discrete card that is a coin toss decided outside the
  application, and the Windows graphics preference was the only way to settle it.
  The Transcription page lists the cards by name now, and the model reloads on the
  one picked without a restart.

  Nothing saved means the discrete card rather than the roomiest one. Measured on
  this machine, the integrated Iris Xe reports 16 GB of shared system memory
  against the 4060's 8 GB of its own, so memory alone would hand the work to the
  slower of the two. Class first, then memory within a class.

  The choice is saved as a name beside an index, because gpu_device is a rank
  among the GPUs and not a device id: it moves the day a card is added or a driver
  stops reporting one, and the name is what finds the card again.

  The cards are read from the ggml device registry, which is the same walk whisper
  does to resolve that rank, so the two agree by construction. It also wraps a
  Vulkan that fails to come up in a catch, which the Vulkan entry points do not,
  and an exception crossing back into Rust would take the process with it. Reading
  it is why whisper-rs now carries raw-api, which re-exports the sys crate. The
  walk only happens in GPU mode: enumerating brings the Vulkan instance up, and a
  machine running on the CPU has no reason to pay for that.
- (recording) Turn the machine down while you talk, instead of pausing it

The pause sent MediaPlayPause blindly at whatever window was in front. It hit
  the wrong application as often as the right one, had no way of knowing whether
  it had paused or resumed, and could not undo a wrong guess. Lowering the render
  endpoint touches everything at once and is exactly reversible.

  A toggle and a slider, down to twenty percent by default, and the volume comes
  back at the stop rather than after the transcription: the speaker has finished
  and the wait is no reason to keep the room quiet. A cancelled recording restores
  too, which the pause did not always manage.

  The level taken before ducking is written to the settings file rather than held
  in memory. If the application dies mid-recording the machine is left quiet with
  nothing in it knowing why, and the next launch is the only thing left that can
  put it back, so that is where it restores.

  Ducking is skipped when the volume already sits at or below the target,
  otherwise stopping would push somebody's volume up.
- (history) Delete a single transcription from its own card

Clearing everything was the only way to get rid of one. The button sits in the
  card footer, appears on hover, and stops the click before it reaches the card,
  which copies: deleting and copying in the same gesture would be the worst of the
  two outcomes.

  No confirmation for a single one. The dialog is there for the acts that take
  many at once, and asking on every line would train the reader to click through
  it.

  The statistics stand, the same way they do after a prune: removing a line from
  the history is not a claim that it never happened.
- (history) Choose how much of it to keep, and actually keep to it

Nothing ever pruned. add_transcription inserted and the database grew for as
  long as the application was used. The only bound was the history page asking
  for 200 rows, which hid the growth rather than limiting it, and the changelog
  entry claiming a cap of 100 described a frontend array that did not survive the
  move to SQLite.

  There is a real setting now, default 100, persisted like the rest and applied
  in the one place the history grows. Choosing a smaller number prunes at once
  rather than at the next dictation, so the list on screen and the database say
  the same thing.

  Zero means everything, which is what the application did before. The page reads
  the kept rows back from the database after a change instead of trimming its own
  list, since guessing which rows went would put the two out of step.

  The statistics are an aggregate in another table and are deliberately left
  alone: dropping old transcriptions must not rewrite what has already been
  counted. There is a test for that, and for the ordering, since this is a DELETE
  on somebody's history.

  Clear all loses its label to match the dashboard, and keeps an accessible name.
- (dashboard) Compare against three hosted APIs, not one

The card quoted a single rate, OpenAI's, and it lived in two places: database.rs
  computed the saving from its own copy while analytics.ts held the copy shown on
  screen. Two constants for one number, free to drift.

  There is one table now, read by both the card and the headline figure. Three
  providers, laid out like the subscription card next to it: the cheap end, the
  one everybody knows, and a major cloud. A spread rather than the three cheapest,
  since a comparison that only picks flattering numbers is not worth showing.

  The headline takes the cheapest of the three, so the saving holds whichever
  provider the reader would have gone with.

  Where it ran goes, and the split bar with it. It answered a different question
  from the one the card asks, and the bar had nothing to compare on the many
  installs that only ever use one mode.

### Maintenance

- (repo) Add the pull request template, and the rules that go with it

The template is what a request answers rather than a body written from memory: what changes, why it was not already like that, what proves it, what it leaves owing, and a checklist that gets ticked truthfully. It also carries the merge button, since the wrong one either forks the tree or throws away the reasoning in the commit bodies.
- (repo) Write down the branch flow, and check dev the way main is checked

Work accumulates on dev and ships from main, so both are protected on the remote: no
  direct push, no force push, no deletion, linear history, and the frontend check green
  before a merge. A feature or a fix takes its own branch off dev and returns by pull
  request; main only ever receives dev, through one pull request that is the deployment.

  The check workflow only ran on pushes to main, so dev would have accumulated without a
  gate of its own between pull requests.
- (installer) Build the wizard in English only

The application, its Rust strings included, is entirely in English. The NSIS
  wizard still offered French and asked which language to install in, which was
  one screen standing between the user and the install for no benefit.

  The installer hooks were already written in English, so nothing else moves.

### Refactoring

- (preferences) Both devices at the top, the companion shortcuts folded at the bottom

Where a sound comes out belongs with where the voice goes in, not buried inside the
  sounds it happens to govern. InputDeviceSection becomes AudioDevicesSection and carries
  the two: same icon, same list, same button to look for devices again, one row each. The
  sounds section keeps the presets and points at where the choice now lives.

  The companion shortcuts were open at all times on a list most installations never fill,
  and they sat in the middle of the page, between the sounds and the system settings. They
  fold away now, shut by default with a count beside the title, and they sit at the bottom
  where a rarely touched list belongs. Add still works from the shut state and opens the
  section on the way.
- (recording) Save the transcription where it is produced

The frontend was persisting what Rust had just transcribed. That is what forced
  a second event: both handlers answered transcription-complete, so the dashboard
  query raced the write and the figures sat one dictation behind. Saving before
  announcing removes the race by construction, and the extra event with it.

  Two columns stop being null. audio_duration_ms and processing_time_ms were
  always null because the frontend cannot know either: it has neither the samples
  nor the clock. Rust has both.

  The source is no longer guessed from the mode the user picked. A server
  dictation that quietly fell back to the local engine was recorded as "server",
  so the badge in the history lied about which engine ran. The branch now carries
  the answer out with the text.

  db_add_transcription goes, along with the two refs that existed only to tell it
  what to write.

### Tests

- (vocabulary) Cover the view, and give its buttons a name

Eleven tests over the whole loop: the empty state, the count, adding one term
  and several from one line, Enter as a submit, the duplicate that must reach
  neither the backend nor the parent, removing one term and clearing the lot.

  Two things the tests forced out into the open. The remove and reorder buttons
  were icons with no accessible name, so a screen reader announced nothing at all
  and a test had nothing to grab; both carry an aria-label now. And the word list
  heading was still "Vos termes", which every earlier scan missed because it sits
  against a JSX expression and the extraction refused any run containing braces.
- Cover the audio buffer, the tone generation and the WAV edges

Twenty-four more tests, on the three modules that are pure enough to exercise
  without a device.

  The audio buffer gets its meter, its spectrum and its ten-minute cap. The meter
  one is worth the line it costs: it reads the tail of the buffer rather than all
  of it, so a long loud recording followed by silence has to fall, or the overlay
  keeps showing speech after the speaker stopped.

  Sound generation gets length, gain and the fade. A buffer that ends at full
  amplitude is heard as a click, and the descending sweep integrates a negative
  term, which is where a NaN would come from and rodio plays those as noise.

  The WAV encoder gets what the previous two tests skipped: clamping past full
  scale, which would otherwise wrap the loudest part of a word into the quietest,
  non-finite input from a misbehaving device, and the header fields Whisper
  resamples from.
- Collect only the tests under src

A git worktree under .claude/worktrees carries its own copy of the tree, so
  vitest found every test twice and reported double the count. Anchoring the
  include at src leaves the nested copies out.
- Cover the analytics helpers and the settings defaults

The analytics side carries the arithmetic behind the comparison card, where a
  wrong figure is not obviously wrong on screen: months are counted as a
  subscription bills them, and a fresh install already owes one.

  On the Rust side the tests pin the defaults the frontend initialises its own
  state from. Those two drifting apart is what put the overlay theme, the overlay
  size and the sound state each in a different place. A test says so now.

  load_settings and save_settings are deliberately not exercised: they resolve
  through ProjectDirs to the real %APPDATA%, so a test run would read and
  overwrite the settings of whoever ran it.
- Stand up the client test harnesses

vitest on jsdom for the frontend, cargo test for the native side. The Rust one
  needed no new dependency, only a script that loads the MSVC environment the way
  the build ones do.

  vitest.config.ts is kept apart from vite.config.ts because that one exports an
  async factory reading TAURI_DEV_HOST and pinning the dev server to port 1421,
  none of which a test run should touch.

  The setup file mocks invoke and listen. jsdom has no Tauri runtime behind it, so
  the real ones throw before a component renders at all. matchMedia and
  ResizeObserver are stubbed for the same reason: jsdom implements neither and the
  overlay and the scroll areas both reach for them.

## [0.7.0] - 2026-08-26

### Bug Fixes

- (window) Open at 1190x750 so the home page fits

At 1200x700 the typing test sat under the fold on the page the application
  opens on. Fifty pixels taller and the whole thing is in the window, with the
  content centred rather than pressed against the top.
- (ui) Give controls a pointer, and stop the window selecting like a page

Tailwind 4 dropped the pointer cursor on buttons, so every control in the
  sidebar kept the arrow and read as inert. One base rule covers the whole
  application, and the classes sprinkled on individual buttons come back out.

  Dragging across the window also painted labels, headings and counters blue,
  which no desktop application does. Selection is off by default and handed back
  where there is something worth copying: the fields you type into, and the
  transcribed text in the history, which carries a .selectable class.
- (analytics) Give Retest room to breathe

It sat against the speed it re-measures, at ten pixels and off the baseline,
  so it read as a superscript rather than a link.
- (activity) Scale the graph against a real day

The intensity ceiling was 700 dictations in a day, a number nobody reaches, so
  every real day landed in the faintest band and a full year of work read as an
  empty grid. Eight gives a first week visible contrast and stops mattering as
  soon as there is a busier day to scale against.
- (installer) Retire the T4lk install, and reclaim the model cache

Tauri keys the uninstall entry on the product name rather than on the bundle
  identifier, so an install of Talk was invisible to the T4lk entry already on the
  machine: Windows would list two applications, keep two shortcuts, and leave the
  old binary on disk. NSIS_HOOK_PREINSTALL now deletes those keys and that
  directory. Never by running the old uninstaller, whose own hook reaches into the
  data directory, which is the whole point of having kept the identifier.

  The uninstall hook also did nothing at all. It deleted
  %APPDATA%\com.avpbynf.t4lk, and nothing is ever written there: on Windows the
  directories crate drops the qualifier, so the real path is %APPDATA%\avpbynf\t4lk.
  Uninstalling therefore left the models behind, a gigabyte and a half of them. It
  now reclaims those, since they download themselves again, and leaves
  settings.json and the history where a reinstall will find them.
- (titlebar) Put the application name before the status

The model or the server state came first and the name second, behind an em
  dash. The name now leads in full colour and the status follows it, in
  parentheses and dimmed.
- (transcription) Drop the slide on page load

slide-enter was on the whole page container, which no other view does, so
  Transcription alone appeared to slide in. The class stays where it belongs, on
  the block that reveals itself in SoundFeedbackSection.
- (history) Confirm before clearing everything

The button wiped the whole history on the click, with no way back. It now opens
  a small modal over the page, dismissed by Escape or by clicking beside it, using
  the words the dashboard already uses for the same question.

### Build

- Generate the installer bitmaps from a script

The two BMP the NSIS wizard displays were composed by hand, so the wordmark
  they carried survived the rename. They now come out of a script that draws
  the real icon and the real Outfit face on the dark theme tokens, and can be
  rebuilt whenever the mark changes.

### Documentation

- Replace the README screenshot with a cleaner capture

The previous one came from PrintWindow, which returns the whole window
  rectangle including the invisible resize border, so it carried a few pixels of
  margin on three sides and none on the fourth. This one is the client area,
  1192x752, and sits square.
- Show the home page in the README

Taken from the running application, at the size it now opens at, and reduced to
  a 256 colour palette: identical to the eye and a little over half the weight.
- Correct which string names the data directory

Four call sites resolve to %APPDATA%\avpbynf\t4lk, where settings.json, t4lk.db
  and better than a gigabyte of models live. The bundle identifier is a separate
  string naming only the WebView2 profile.
- Local and server are two modes, not a mode and a fallback

TranscriptionMode defaults to Local, and the wizard's first screen offers the
  two as equal choices. The README said server first and local as a rescue, which
  is the shape of one toggle inside server mode, not the shape of the product. It
  now says what each mode answers: a card in this machine, or a card in another
  one shared by everybody.

  The build table also named six tools and linked one. Each has its download page
  now, and the winget line that installs all but Visual Studio.
- Give the path budget instead of a bare example

The build writes 219 characters below the target directory, so the name of that
  directory is the whole budget: about 40 characters, which no path under
  Documents leaves.
- Name the path-length fix the build actually needs

The advice was to move the checkout. The target directory is the half that
  grows, and from a 48 character checkout the default one already crosses the
  limit. Both files now point at CARGO_TARGET_DIR, and record that the message
  which comes back is MSBuild's MSB4184, not anything mentioning CMake.
- Rewrite the README as the app's front door

It opened on what the app is built with. It now opens on why anyone would want
  it, and says what the local engine is for rather than only that it exists.
  CLAUDE.md gains the two traps this rename left behind, and the LLVM requirement
  that makes bindgen panic about a file belonging to nobody.
- Record that the Rust side has no local feedback loop

### Features

- (home) Make the statistics page a home page

It was already the landing view, and it opened on a report. It now opens on
  whether the shortcut will produce text right now: the mode in use, the model or
  the server behind it, the keys to hold, and the last thing dictated.

  The activity graph sits above the period selector and is deliberately never
  filtered. It is always the whole year, and putting the selector under it is
  what makes that readable rather than surprising.
- (analytics) Compare against what a subscription would cost

The API comparison answers what the audio would have cost to send somewhere.
  This answers the other question, which is what the alternatives charge to sit
  on the machine: Wispr Flow, Dragon Professional and superwhisper, times the
  months since the first dictation.

  Their prices rot, so they live in one place with the month they were checked,
  and that month is printed under the card. Mac-only tools are left out: a
  comparison against something that does not run on Windows would flatter Talk
  and mean nothing.
- (analytics) Let the summary answer over a period

db_get_analytics_summary took a typing speed and nothing else, so every figure
  on the page was a lifetime total. It now takes a window in days, today
  included, and null still means everything.

  It also returns two dates. firstDay is the earliest day carrying activity,
  which daily_stats keeps across a history clear, so it is the real start of use
  rather than the oldest row still kept. periodStart is the window's own start,
  and what a subscription would have billed is counted from whichever of the two
  comes later.
- (overlay) Bring back the size selector

The three sizes existed in Rust and set_overlay_size() really resized the
  window, but no control was exposed and App.tsx forced small on every mount. The
  selector sits next to the theme, and the forcing is gone: the window is already
  built from the persisted size at startup.

  show_overlay() built its window at 200x80, a size matching no OverlaySize
  variant, so an overlay that had to be re-created came back ignoring the setting.
  It reads the setting too now.
- (settings) Default to beep feedback and clipboard preservation

### Maintenance

- (release) V0.7.0

Version bumped in the four files that have to agree, Cargo.lock included, and
  the changelog rebuilt from the history by git-cliff.
- Rename the product from T4lk to Talk

The bundle identifier, the config and data directories and the history
  database keep their com.avpbynf.t4lk spelling. Renaming those would make
  every existing install lose its settings, its history and its downloaded
  model, for a string nobody reads.

  AppTheme keeps a serde alias for each of its two former variants for the
  same reason: load_settings() drops the whole file on a parse error, so a
  settings.json still holding "t4lk-dark" would take the server URL and the
  shortcuts down with it.

### Refactoring

- (i18n) The strings Rust sends to the screen

The frontend sweep could not see these. Twelve model descriptions and three
  accelerator ones land straight in the cards the user picks from, and the delete
  refusal lands in a toast. Nothing French is left in the client now.
- (i18n) The rest of the interface in English

Twenty files: the setup wizard, the preferences and their sections, the
  vocabulary, the history, the transcription tabs, the model and GPU cards, and
  the weekday labels the chart reads out of Rust.

  PreferencesView was also declared as PreferencesView with accents on the
  identifier itself, not only on its labels.

  Dates and counts go through UI_LOCALE rather than the system, so the history
  stops saying "aujourd'hui" in an English window.
- (home) One typeface, no history line, and the graph folded away

Five things, all from watching it run.

  The numbers were set in JetBrains Mono while everything around them was Outfit,
  which read as two designs sharing a card. One typeface throughout.

  The last dictation had its own row under the status strip. It said what the
  history page says, on every visit, and it put whatever was just dictated on
  screen for anyone walking past.

  The activity graph moves to the bottom, above the typing test, and opens on a
  chevron rather than being shown. It is the whole year whatever the period above
  says, so it answers a different question, and on a young history it is a year of
  empty squares nobody asked to see.

  The titlebar loses its status dot, which the home page now carries in words, and
  gains room between the name and the model.

  Figures also follow the interface rather than the machine: on a French Windows
  an English page was printing "2 733" with a narrow space and "août 2026".
- (analytics) Rewrite the cards in English, and stop opposing the modes

The four counters become four cards, each carrying the scope of its own figure
  rather than leaving them to be read as one.

  Where it ran no longer paints a split bar when only one mode has ever been
  used. Plenty of installs are local only or server only, and a bar cut at 100/0
  claims a balance that does not exist: it says which one, in a line, instead.

## [0.6.0] - 2026-08-26

### Bug Fixes

- (installer) Redraw the header, it carried the previous product name
- (installer) Redraw the sidebar, it carried the previous product name
- (shortcuts) Remove an unused local
- (history) Remove an unused import
- (overlay) Drop the unread size value, keep its setter
- (ui) Drop the unread isRecording value, keep its setter
- (ui) Give useRef an initial value, required by React 19 types
- (csp) Allow any HTTPS server instead of a single host
- (setup) Drop the hardcoded default server URL
- (ui) Drop the hardcoded default server URL
- (state) Drop the hardcoded default server URL
- (settings) Drop the hardcoded default server URL
- (installer) Skip the VB-Cable setup when its payload is absent
- (build) Detect any Visual Studio 2022 edition, fail loudly when none
- (i18n) Add missing French accents across all UI strings

- App: Préférences, Serveur connecté, Non prêt, Aucun modèle
  - Preferences: Préférences, système
  - InputDevice: périphérique, Défaut système, Rafraîchir
  - LocalTab: Modèles, téléchargé, Quantifiés
  - ServerTab: Vérification, Connecté, testé, Délai, modèle
  - ModelCard: Téléchargement
  - GpuSelector: Accélération, générique
  - TimeSaved: Transcription réelle
  - Analytics: all TYPING_SENTENCES with proper accents

### Build

- Commit the bun lockfile
- Pin the Tauri npm packages to the Rust crate minor

### Documentation

- Record how the path limit actually surfaces
- (build) Explain why cargo forces the Ninja generator
- Remove the throwaway redesign prompt
- Remove the throwaway overlay design prompt
- Add repository conventions
- Add a README
- Add the MIT licence

### Features

- (ui) Add motion animations, improve history UX, fix French accents

- Add motion library for micro-interactions
  - History: click card to copy, AnimatePresence for new entries, ghost
    clipboard icon feedback, whileTap press effect
  - History: unified layout with persistent header, disabled clear button
    when empty, simplified empty state
  - History: hide model badge for server transcriptions
  - Button: add cursor-pointer globally
  - Fix all missing French accents in UI strings (UTF-8)
- (dashboard) Redesign analytics with heatmap and compact stats

- Replace 2x2 card grid with single-row compact data strip (colored dots)
  - Replace bar chart with SVG yearly heatmap grid (365 days, 5 intensity levels)
  - Add month and day-of-week labels to heatmap
  - Add db_get_yearly_activity Rust command querying daily_stats for past 365 days
  - Add YearlyDayActivity type in Rust and TypeScript
  - Intensity thresholds: absolute baseline 700 with user-adaptive scaling
  - Rename nav label from Accueil to Dashboard
- (database) Migrate transcription history to SQLite with rusqlite

Replace JSON file persistence with a SQLite database using rusqlite.
  Add analytics SQL queries for the analytics view. Remove legacy JSON
  persistence code from settings.rs.
- (ui) Add analytics home page with stats dashboard

Add analytics home page as the default view with:
  - stats cards (transcription count, time saved, words transcribed, accuracy)
  - activity chart showing usage over time
  - cost comparison tracker vs OpenAI Whisper API
  - time savings tracker with cumulative metrics
  - typing speed calibration game for baseline measurement

### Maintenance

- Stop ignoring the bun lockfile
- Hide the line-ending normalization from blame
- Ignore the VB-Cable driver payload
- Normalize line endings to LF
- (release) Bump version to v0.6.0

### Performance

- (sound) Migrate audio feedback from Web Audio API to Rust

- Add rodio-based SoundEngine with pre-computed PCM buffers in RAM
  - Play sounds directly in Rust (start/stop/cancel recording) before
    emitting JS events, eliminating IPC + WebView latency
  - Keep OutputStream alive in dedicated parked thread (cpal !Send)
  - Pre-render overlay DOM with visibility:hidden instead of return null
  - Delete src/lib/audio.ts, remove JS sound refs/effects from App.tsx
  - Preview sounds in settings via invoke("preview_sound") instead of JS
  - Add tauri:check script to package.json for cargo check via vcenv

## [0.5.0] - 2026-03-22

### Bug Fixes

- (hotkeys) Eliminate closure accumulation on enable/disable cycles

Replaced per-shortcut on_shortcut() calls with single
  Builder::with_handler() dispatch pattern, zero closure allocation on
  enable/disable/update cycles. Removed console.log from hot path.
- (audio) Prevent memory leaks in resample buffers and web audio

- Fix resample_buffer drain skipped when consumed >= len (audio + virtual mic)
  - Cap virtual mic ring buffer at 96k samples to prevent unbounded growth
  - Disconnect AudioContext oscillator/gain nodes after playback ends
  - Add clearTimeout cleanup on InputDeviceSection unmount
- (ui) Clean up ServerTab text colors, remove SSE box

- Remove SSE streaming info box (unnecessary)
  - Fix Token API label/description using text-muted instead of
    text-muted-foreground (matching URL label style)
  - Fix fallback description same issue
  - Remove unused Activity import
- (ui) Homogenize spacing, colors, typography

- Replace hardcoded emerald-500/red-500 with design tokens
    (bg-success/bg-destructive) in MeetingModeSection
  - Add missing space-y-4 to LocalTab and ServerTab card containers
  - Align info-box opacity to /10 + /20 (ServerTab SSE box)
  - Align GpuSelector icon background opacity to /10
  - Standardize label sizing to text-sm font-medium across all
    preference sections (InputDevice, System, MeetingMode)
  - Normalize placeholder opacity in CompanionShortcutsSection
  - Unify empty state border to border-border-card
- (ui) Use shadcn Select component for input device dropdown
- Use direct import for find_vbcable_device in router
- Remove unused exports and dead code warnings
- (ui) Swap shortcuts and recording mode section order
- (ui) Design audit polish pass

- Add section header to ShortcutsSection (was the only section without)
  - Normalize SystemSection: remove icon badges from last 2 items to
    match the plain style of the first 2
  - Unify kbd sizing: remove inline overrides in KeyCaptureField, use
    global kbd style consistently
  - Fix CompanionShortcuts padding to match other sections (p-5)
  - Switch OverlaySection to cn() instead of template literals
  - Add focus-visible ring + keyboard support to KeyCaptureField
  - Fix accent: Theme → Thème
- (ui) Cursor-pointer on all interactive elements + French accents

- Add cursor-pointer to all buttons, selects, clickable elements
    across all preference sections
  - Fix missing French accents: Démarrage, Arrêt, assigné, etc.
- (ui) Replace trash icon with X for companion shortcut delete
- (ui) Remove key capture border + reduce card bottom padding
- (ui) Always-visible delete button + remove row borders

Delete button no longer hidden on hover. Remove border/background
  from rows for a cleaner inline look: background only shows when
  dragging.
- Play stop sound on recording cancellation
- Event listeners lost in StrictMode + companion UI polish

- Reset hasRegisteredListeners flag in cleanup so listeners survive
    React 18 StrictMode unmount/remount cycle (fixes sounds + companion
    shortcuts not firing)
  - Extract fireCompanionShortcuts helper with error logging
  - Redesign companion shortcuts: color-coded trigger badges (green
    start, amber stop, cyan both), segmented trigger control in edit
    mode, hover-reveal actions, separator dots, better empty state
- (ui) Key-capture for companion shortcuts + cancel trigger

- Replace text input with proper key-capture interface (same as main
    shortcuts section) for companion shortcut key assignment
  - Companion shortcuts now fire on recording cancellation (stop trigger)
- Companion shortcuts on cancel + titlebar visibility

- Fire companion shortcuts (trigger "stop"/"both") when recording is
    cancelled, matching the behavior of normal stop
  - Change titlebar title from text-muted (nearly invisible) to
    text-muted-foreground for proper contrast
- (build) Upgrade whisper-rs 0.16, add Ninja generator

- Upgrade whisper-rs 0.14 -> 0.16 (bindgen 0.72 fixes opaque structs)
  - Add .cargo/config.toml with CMAKE_GENERATOR=Ninja (bypasses vswhere)
  - Add rust-toolchain.toml pinning Rust 1.90.0
  - Commit Cargo.lock (removed from .gitignore)
  - Add tauri:dev/build/clean scripts with auto vcvarsall.bat
  - Adapt transcription code to whisper-rs 0.16 API changes
- Correct IAudioMeterInformation import path for windows crate 0.62

Move import from Win32::Media::Audio to Win32::Media::Audio::Endpoints
  and re-add Win32_Media_Audio_Endpoints feature flag in Cargo.toml.
- (hotkeys) Resolve IAudioEndpointVolume build errors

Add missing windows crate features Win32_System_Com_StructuredStorage
  and Win32_System_Variant in Cargo.toml, and fix import path from
  Audio to Audio::Endpoints in hotkeys/mod.rs.
- (tray) Remove duplicate tray icon and redundant show menu item

Remove trayIcon from tauri.conf.json which was duplicating the icon
  already created by TrayIconBuilder in lib.rs. Also remove the redundant
  "Show" menu item from the tray menu since left-clicking the icon already
  shows the window.
- (clipboard) Use direct typing fallback for terminals

SendInput (enigo Ctrl+V) is ignored by WinUI apps like Windows Terminal.
  When the active window domain is "terminal", use enigo.text() for direct
  character typing instead of clipboard + Ctrl+V simulation.
- Align default server URL in React state to stt.example.com
- (history) Limit transcription history to 100 entries

Slice the array after prepend to keep only the 100 most recent
  transcriptions, preventing unbounded growth of history.json.
- (ui) Restore French UTF-8 accents and clean up GPU selector

- Add missing accents to all UI strings in 5 views (PreferencesView,
    TranscriptionView, VocabularyView, HistoryView, SetupWizard)
  - Swap CPU/Vulkan order in GPU selector (TranscriptionView)
  - Remove stale overlay size info message (PreferencesView)
- Remove dead screenshot/claude refs in hotkeys

Clean hotkeys/mod.rs: remove all screenshot capture logic, Claude API
  enhancement calls, and references to deleted AppState fields.
  Fix index.html title (Whisper Flow → T4lk).
  Fix overlay preview label (200x60 → 220x60 to match settings.rs).

### Documentation

- Remove orphan client cleanup design spec

Implementation complete, spec superseded by code.

### Features

- (ui) Add app theme system with 7 predefined themes

Add a complete theming system for the app appearance, independent
  from the overlay themes. Uses CSS custom property overrides via
  data-theme attribute on <html>. No external library needed.
- (ui) Split Appearance page, redesign titlebar + mic

- Add AppearanceView page with Overlay section
  - Move SoundFeedback back to PreferencesView (recording behavior)
  - Reorder Preferences: Mic, Mode, Shortcuts, Sounds, Companion,
    Meeting, System
  - Add Appearance nav item (Palette icon) in sidebar
  - Move status dot from sidebar to titlebar with contextual label
    (model name or server status before app name)
  - Redesign InputDeviceSection with colored icon, refresh button
    with spin animation, default device name detection
  - Add cursor-pointer to ServerTab refresh button
- Add input device selector in preferences

Enumerate available input devices via cpal and let the user choose
  which microphone to use for STT capture. Defaults to system default.
  New dropdown in preferences page, persisted in settings.
- Add virtual mic meeting mode (VB-Cable routing)

Route real microphone through VB-Cable so meeting apps hear silence
  during STT recording. Adds virtual_mic module (detector, router,
  controller), meeting mode toggle in preferences, muted indicator
  in overlay, and NSIS hook for VB-Cable silent install.

  New files: virtual_mic/{mod,detector,router,controller}.rs,
  MeetingModeSection.tsx, .gitattributes (LFS for exe resources).
  Modified: lib.rs, settings.rs, hotkeys/mod.rs, nsis-hooks.nsh,
  PreferencesView.tsx, OverlayPage.tsx.
- (ui) Reorder companion shortcuts with up/down buttons

Add chevron up/down buttons in the edit mode action bar to move
  shortcuts in the list. Buttons are disabled at list boundaries.
- (overlay) Multi-arc glow effect with customizable themes

- Replace single rotating arc with 3 independent arcs at different speeds
    and directions (one counter-clockwise), creating a Gemini-like effect
  - Arcs almost freeze at silence, come alive with audio (sine wobble for
    organic variation, smoothed audio decay for gradual slowdown)
  - Add 6 theme presets (Aurora, Sunset, Ocean, Neon, Frost, Neutral) that
    control border glow colors AND interior UI (mic, bars, timer, dots)
  - Theme selection UI in Preferences with color preview dots
  - Real-time theme switching via Tauri event (no restart needed)
  - Fix overlay transparency: body bg override, shadow(false), clip ambient
    glow to pill shape to prevent dark rectangle artifacts
- (ui) Overlay polish, sidebar icons, cleanup dead code

- Overlay: animated enter/exit, rotating glow border, spectrum bars
    with GPU compositing, processing dots, elapsed timer
  - Sidebar: simplified nav with Cpu/BookA icons, bottom status dot
  - Remove unused type_text_direct from clipboard module
  - Suppress dead_code warnings on server transcription structs
  - Simplify SystemSection (remove overlay description card)
  - Add vcenv.bat script, simplify tauri:dev/tauri:build scripts
  - Add design docs and specs
- (ui) Wire sound feedback (R3) and companion shortcuts (R4)
- (backend) Add sound, companion shortcuts, server token settings
- Add Web Audio API sound synthesis engine
- (ui) Reorganize sidebar with top/bottom groups

Split nav items into content group (History, Vocabulary) on top
  and settings group (Transcription, Preferences) on bottom,
  separated by a subtle divider line.
- (ui) Preferences -- shortcuts side-by-side, simplify overlay
- (ui) Flatten transcription page into single scrollable view

Replace 3-tab structure with 2-mode selector (Local/Server), add token
  field and fallback toggle to ServerTab, delete EngineTab and
  EngineModeCard. App.tsx gains serverToken state and passes it through.
- (ui) History -- local vs server source badge
- (ui) Vocabulary -- info box on top, fix dedup and space handling
- (ui) Simplify titlebar -- remove recording indicator
- (client) Simplify vocabulary, remove bundled model, add mic mute

- Remove bundled 574MB GGML model: download from HuggingFace at first launch
  - Simplify vocabulary system: remove language-based vocabularies, add 9 default terms in settings, reduce setup wizard from 5 to 3 steps
  - Add mic mute feature: mute system microphone during recording via Windows Core Audio API
  - Simplify media pause to plain play/pause toggle (remove unreliable GSMTCS API)
  - Fix build warnings in transcription and window modules
- (release) Production packaging v0.3.0

Bundle large-v3-turbo-q5_0 GGML model (574 MB) via Git LFS for
  offline-first experience. Configure NSIS installer (currentUser,
  French/English). Copy bundled model to user data dir at startup.
- Migrate client from Whisper Flow to T4lk

Rebrand all identifiers (com.avpbynf.t4lk), titles, and config paths.
  Remove Claude API integration, screenshot capture, and server formatting.
  Adapt server_transcription.rs to new OpenAI-compatible API
  (/v1/audio/transcriptions/stream, no auth). Add T4lk business vocabulary.
  Delete 7 dead files (claude_api.rs, screenshot/mod.rs, 5 views).

  -944 lines removed, +184 lines added across 25 files.
- Initial t4lk-client from Whisper Flow

Copy of whisper-client source code (Tauri v2 + React 19).
  Desktop STT app with local/server transcription, vocabulary, overlay.

### Maintenance

- (release) Bump version to v0.5.0
- Update Cargo.lock
- Rebrand to T4lk with com.avpbynf.t4lk identity

- Rename t4lk/T4lk to t4lk/T4lk everywhere
  - Update app identifier to com.avpbynf.t4lk
  - Replace the legacy lib with t4lk_lib
  - Empty default vocabulary (remove T4lk terms)
  - Remove all T4lk branding references
- Bundle VB-Cable driver for NSIS installer

Add full VBCABLE_Driver directory (setup exe + driver files) to
  resources. Update NSIS hook to extract the entire folder before
  running setup. Add LFS tracking for binary files (.exe, .sys, .cat)
  and gitignore exception for bundled executables.
- (nsis) Add post-uninstall hook to clean user data
- (release) Bump version to 0.4.0
- (nsis) Add installer branding images

Header (150x57) and sidebar (164x314) with purple gradient
  matching app icon, wave motif, and T4lk text.
- Add git-cliff configuration and initial CHANGELOG
- Change authors from personal to T4lk

### Performance

- (overlay) Warm up WebView2 at startup for instant show

Create overlay window visible (not hidden) at startup so WebView2
  eagerly loads HTML/JS/React. Hide after 500ms once rendering pipeline
  is initialized. Move transparent background override to an inline
  <script> in index.html (runs before CSS, prevents dark flash).
  Remove the runtime useEffect style injection from OverlayPage.
- (recording) Optimize start timing by reordering operations

Mute virtual mic first (instant AtomicBool flip), start audio capture
  immediately, then show overlay. Removes 50ms sleep and overlay
  re-creation (~100ms+ WebviewWindow build). Overlay is pre-created at
  startup and simply shown/hidden, never recreated in the hot path.
  Net latency reduction: ~150ms+ on recording start.

### Refactoring

- (ui) Remove fixed headers, scroll titles

- Remove fixed header bars from all 4 views (History, Vocabulary,
    Transcription, Preferences) and move title/description/actions
    into the scrollable content area with a subtle separator
  - Move "Décharger" button from TranscriptionView header into
    ModelCard component (next to the loaded model)
  - Pass onUnload prop through LocalTab to ModelCard
  - Clean up unused imports (Activity, X, Button) in TranscriptionView
  - Fix HistoryView spacing: space-y-6 layout, space-y-3 cards only
  - Fix ShortcutsSection card wrapper consistency
- Replace VB-Cable with open-source Virtual Audio Driver

Swap donationware VB-Cable (not compatible with commercial use)
  for VirtualDrivers/Virtual-Audio-Driver (MIT license, signed via
  SignPath.io). Driver package reduced from ~1.1MB to ~100KB.
  NSIS hook now uses pnputil instead of setup exe.
  Rename all VBCable references to VirtualAudio across Rust and
  frontend code.
- (ui) Click-to-capture keys with cancel X, no pencil
- (ui) Auto-save + drag-to-reorder companion shortcuts

- Remove edit/view modes, draft state, save/cancel buttons: all
    fields are inline-editable and auto-save on change
  - Add drag-to-reorder with @dnd-kit (same pattern as vocabulary)
    using GripVertical handle
  - Delete button appears on hover
  - Each row: [grip] [label input] [trigger dropdown] [key capture] [trash]
- Extract KeyCaptureField shared component

- Create reusable KeyCaptureField with pencil-to-capture pattern,
    proper modifier+key validation, disable/enable global shortcuts
  - Remove all duplicated key capture logic from CompanionShortcutsSection
  - Fix bug: capture now stays active until a valid combo is pressed
    (modifier+key), not just one keypress
- (ui) Pencil button to enter key capture mode

Keys are displayed as read-only kbd tags in edit mode. Click the
  pencil icon to enter capture mode, press a key combo, and it
  auto-exits capture. Avoids accidental key capture when clicking
  into the edit panel.
- (ui) Use Select dropdown for companion trigger mode

Replace segmented button group with a Select dropdown for the
  start/stop/both trigger selection in companion shortcut edit mode.
- (ui) Reorder action bar in companion shortcut edit
- (ui) Click-to-edit companion shortcuts, remove buttons

Entire row is clickable (cursor pointer) to enter edit mode.
  Remove edit/delete hover buttons from view mode, delete is available
  in edit mode action bar.
- (ui) Reorder companion shortcut fields

Display order changed to Label > Trigger > Keys in both view and
  edit modes for better readability.
- (ui) Compact inline companion shortcuts layout

View mode: single row with trigger badge, kbd tags, label, edit/delete.
  Edit mode: inline trigger buttons, key capture, label input, save/cancel.
  New shortcut auto-enters edit mode on creation.
- (ui) Split PreferencesView into 5 section components

Extract RecordingModeSection, ShortcutsSection, CompanionShortcutsSection,
  SoundFeedbackSection, and SystemSection into src/views/preferences/.
  PreferencesView.tsx reduced from 584 to ~100 lines.
- (transcription) Split TranscriptionView into sub-components

Split 770-line TranscriptionView into focused sub-components:
  - views/transcription/ orchestrator + EngineTab, LocalTab, ServerTab
  - components/ shared: EngineModeCard, GpuSelector, ModelCard

  Lift serverStatus to App.tsx, fix sidebar status dot to reflect
  correct state per transcription mode (local/server/hybrid), and
  migrate remaining inline OKLCH values to semantic design tokens.
- (ui) Migrate inline OKLCH values to semantic design tokens

Add 16 semantic OKLCH tokens (surface hierarchy, border hierarchy, mode
  accent colors) to index.css @theme. Migrate all inline OKLCH values and
  Tailwind named colors (blue-*, purple-*, amber-*, red-*, cyan-*) to
  semantic tokens across 6 files.

  Zero visual regression - all values map to their exact OKLCH equivalents.
- (client) Remove context detection module and fix media controls

Remove context_detection module (window/IDE/framework detection, ~1523 lines).
  Remove mute mic feature (muted cpal capture, self-sabotaging).
  Fix media pause: check audio playback via IAudioMeterInformation before
  sending MediaPlayPause, resume at recording stop instead of post-transcription.
  Remove active-win-pos-rs, toml, once_cell dependencies.

  -1966 lines deleted.
- (client) Remove dead code and simplify GPU to Vulkan + CPU

- Remove build_vocabulary, clipboard image functions and orphan tests
  - Simplify GPU stack: drop CUDA/Metal/IntelSYCL, keep Vulkan + CPU only
  - Remove dead overlay states (capturing, enhancing, server_formatting)
  - Fix: stop sending programming language name as Whisper language code
  - Update CSP for HTTP local/LAN, change dev port to 1421
  - Clean TranscriptionView, SetupWizard, App.tsx, OverlayPage

  Adds design spec docs/specs/2026-03-16-client-cleanup-design.md.

---
*Generated by [git-cliff](https://git-cliff.org/)*
