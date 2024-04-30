# AnyList Desktop for Windows and Linux

This is a quick [Tauri](https://tauri.app) based wrapper around the excellent [AnyList](https://www.anylist.com) application.

No ownership or interest in AnyList by the author is implied by the creation of this wrapper. It was created largely out of self-interest.

## Why this wrapper?

I really like AnyList. They currently ship Mac, Apple IOS, and Android versions of their application. Desktop users other than Mac users are left with the web version, which requires a subscription to access. This wrapper does not bypass that requirement - it is simply a convenience for those who may wish to have an "app" rather than a webpage. 

The web version of AnyList works quite well; however, I do not like leaving browser tabs pinned or creating site-specific browsers.

The AppImage and .deb run correctly on Ubuntu 22.04. On 24.04, you will need to run the AppImage as the .deb has dependency issues. I've resolved those issues in release 1.2, but that is not yet ready for release.

I am working on a Windows release. This code will compile and run correctly under Windows 11; I am not testing on older versions.

The tauri2.0-beta branch is tagged to release 1.2. I am holding this release as a source-only release until Tauri 2.0 comes out of beta, so you will have to build it yourself. No new features, it just moves to the v2 Tauri release.

I do not build a Mac version because AnyList ships their own.

## Wrapper only (C) 2023 by Steve Luzynski <steve@luzynski.net>
