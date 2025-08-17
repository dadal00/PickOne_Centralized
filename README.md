# PickOne

[![Docker](https://img.shields.io/badge/Docker-2496ED?logo=docker&logoColor=fff)](https://www.docker.com/)
[![Rust](https://img.shields.io/badge/Rust-%23000000.svg?e&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Svelte](https://img.shields.io/badge/Svelte-%23f1413d.svg?logo=svelte&logoColor=white)](https://svelte.dev/)
[![TypeScript](https://img.shields.io/badge/TypeScript-3178C6?logo=typescript&logoColor=fff)](https://www.typescriptlang.org/)
[![Redis](https://img.shields.io/badge/Redis-%23DD0031.svg?logo=redis&logoColor=white)](https://redis.io/)
[![Oracle Cloud](https://custom-icon-badges.demolab.com/badge/Oracle%20Cloud-F80000?logo=oracle&logoColor=white)](https://www.oracle.com/cloud/)
[<img src="demos/badges/caddy.png" width=65 alt="Caddy badge">](https://caddyserver.com)
[<img src="demos/badges/scylla.png" width=65 alt="ScyllaDB badge">](https://www.scylladb.com/)
[<img src="demos/badges/meilisearch.png" width=105 alt="Meilisearch badge">](https://www.meilisearch.com/)
[<img src="demos/badges/grafana.png" width=105 alt="Grafana badge">](https://grafana.com/)

_PickOne is designed to be a centralized hub of various services._

_Credits to @shadEdkr for the frontend design and service ideas._

| Service                               | Status                                                                        | Link                                                       |
| ------------------------------------- | ----------------------------------------------------------------------------- | ---------------------------------------------------------- |
| [BoilerCuts](#boilercuts)             | ![Active](https://img.shields.io/badge/status-active-brightgreen)             | [https://t.me/boilercuts_bot](https://t.me/boilercuts_bot) |
| [BoilerSwap](#boilerswap)             | ![Active](https://img.shields.io/badge/status-active-brightgreen)             | [https://pickone.cc/swap](https://pickone.cc/swap)         |
| [RateMyPWLHousing](#ratemypwlhousing) | ![In Development](https://img.shields.io/badge/status-in--development-FFA500) | [See Demo](#ratemypwlhousing)                              |

Home Website: [https://pickone.cc](https://pickone.cc)

## Table of Contents

- [Built With](#built-with)
  - [Technologies](#technologies)
  - [Architectural Diagram](#architecture-diagram)
  - [Architecture Explanation](#architecture-explanation)
- [Services](#services)
  - [BoilerCuts](#boilercuts)
  - [BoilerSwap](#boilerswap)
  - [RateMyPWLHousing (WIP)](#ratemypwlhousing)
    - [Dark Mode](#dark-mode)
- [Additional Demos](#additional-demos)
  - [Caddy Dashboard](#caddy-dashboard)
  - [Meilisearch Dashboard](#meilisearch-dashboard)
  - [Redis Dashboard](#redis-dashboard)
  - [ScyllaDB Dashboard](#scylladb-dashboard)
- [Local Reproduction (WIP)](#local-reproduction)

## Built With

### Technologies

#### Deployment

[![Oracle Linux](https://custom-icon-badges.demolab.com/badge/Oracle_Linux-F80000?logo=oracle&logoColor=fff)](https://www.oracle.com/linux/) [![Oracle Cloud](https://custom-icon-badges.demolab.com/badge/Oracle%20Cloud-F80000?logo=oracle&logoColor=white)](https://www.oracle.com/cloud/) [<img src="demos/badges/caddy.png" width=65 alt="Caddy badge">](https://caddyserver.com) [![Debian](https://img.shields.io/badge/Debian-A81D33?logo=debian&logoColor=fff)](https://www.debian.org/) [![Docker](https://img.shields.io/badge/Docker-2496ED?logo=docker&logoColor=fff)](https://www.docker.com/)

- VPS (or **Virtual Private Server**) hosted through Oracle Cloud running Oracle Linux
- Caddy used to **reverse proxy into our host machine** from VPS
- **Host machine** running Debian using **Docker containers**

#### Backend + Microservices

[![Rust](https://img.shields.io/badge/Rust-%23000000.svg?e&logo=rust&logoColor=white)](https://www.rust-lang.org/) [<img src="demos/badges/scylla.png" width=65 alt="ScyllaDB badge">](https://www.scylladb.com/) [<img src="demos/badges/meilisearch.png" width=105 alt="Meilisearch badge">](https://www.meilisearch.com/) [![Redis](https://img.shields.io/badge/Redis-%23DD0031.svg?logo=redis&logoColor=white)](https://redis.io/)

- **Backend** built in Rust
- ScyllaDB as the **database**
- Meilisearch as the **search engine**
- Redis as the **cache**

#### Frontend

[![TypeScript](https://img.shields.io/badge/TypeScript-3178C6?logo=typescript&logoColor=fff)](https://www.typescriptlang.org/)
[![Svelte](https://img.shields.io/badge/Svelte-%23f1413d.svg?logo=svelte&logoColor=white)](https://svelte.dev/)

#### Devops

[<img src="demos/badges/grafana.png" width=105 alt="Grafana badge">](https://grafana.com/) [<img src="demos/badges/prometheus.png" width=105 alt="Prometheus badge">](https://prometheus.io/)

### Architecture Diagram

[<img src="demos/diagram/architecture.png" width="350" alt="Architecture Diagram"/>](https://www.figma.com/design/3TCMv4E68enOcQ3quqRtO4/pickone?node-id=0-1&t=GrwhKBXnhd69lmop-1)[<img src="demos/diagram/legend.png" width="200" alt="Legend"/>](https://www.figma.com/design/3TCMv4E68enOcQ3quqRtO4/pickone?node-id=0-1&t=GrwhKBXnhd69lmop-1)

### Architecture Explanation

The diagram illustrates the different communication flows from one piece of technology to another.

#### User

1. The user first comes from the **internet** and accesses our **public endpoint**.

2. This endpoint is our **public domain url** which is routed to our **Oracle Cloud VPS** instance.

3. From there, we use a **reverse proxy** to tunnel into our **host machine**.

- These two devices are **connected using a VPN** to ensure **encrypted traffic** and **to hide entrypoints** into our host machine.

4. After tunneling into our host machine, we do **another reverse proxy using Caddy** to route them to the appropriate service

- In this case it is the **Svelte container** hosting the website.

#### Internal

To reach our backend services, we similarly use the reverse proxy, **Caddy**, on the host machine to route them accordingly.

1. A request reaches **Caddy** or the **host machine's reverse proxy**.

2. We perform a **JWT check** to ensure the request **first reached our frontend** or **originates from our frontend**.

- This **protects our backend** by **authenticating requests** at the **proxy level** before even touching our backend.

3. Next, the request is routed either to our **Rust backend** or **Meilisearch search engine**.

4. Remaining services or connections are performed **internally**.

- **Rust backend** communicates with **ScyllaDB database**, **Redis cache**, or **Meilisearch search engine**.

## Services

Each service includes a brief overview covering:

- Why did we make this?
- How does it work?

Demos are included at the end.

### [BoilerCuts](https://t.me/boilercuts_bot)

[![Active](https://img.shields.io/badge/status-active-brightgreen)](https://t.me/boilercuts_bot)

#### Why did we make this?

Purdue attracts visitors for many reasons—game days, campus tours, or sightseeing. BoilerCuts makes it easy for them to capture and remember their experiences with BoilerMaker-themed photo strips.

#### How does it work?

Using our Telegram chatbot, visitors can upload four photos to create a BoilerMaker-themed photo strip. The photos are stored for 24 hours, and once processed, a QR code and link let users download the final picture, which is also available for 24 hours before deletion.

_See the sample below._

<img src="demos/cuts/part1.jpg" width=300 alt="BoilerCuts start picture">
<img src="demos/cuts/part2.jpg" width=300 alt="BoilerCuts processing picture">
<img src="demos/cuts/sample.jpg" width=300 alt="BoilerCuts sample picture">

### [BoilerSwap](https://pickone.cc/swap)

[![Active](https://img.shields.io/badge/status-active-brightgreen)](https://pickone.cc/swap)

#### Why did we make this?

Moving in and out is a major part of college life, and many perfectly usable items—furniture, kitchenware, and school supplies—often get thrown away. BoilerSwap was created to give Purdue students an easy way to share and discover these items, reducing waste and saving money.

#### How does it work?

Verified Purdue students can post items they no longer need, while anyone else can browse the website to see available items and their locations.

<img src="demos/swap/home.jpg" width=600 alt="BoilerSwap home page">
<img src="demos/swap/search.gif" width=600 alt="BoilerSwap search page">
<img src="demos/swap/login.jpg" width=600 alt="BoilerSwap login page">
<img src="demos/swap/post.jpg" width=600 alt="BoilerSwap post page">

### RateMyPWLHousing

![In Development](https://img.shields.io/badge/status-in--development-FFA500)

#### Why did we make this? + How does it work?

For students new to Purdue, housing options can feel like a black box. RateMyPWLHousing makes it easy to browse and share anonymous feedback, helping future residents know what to expect.

<img src="demos/housing/home.jpg" width=600 alt="Housing home page">
<img src="demos/housing/search.jpg" width=600 alt="Housing search page">
<img src="demos/housing/overview.jpg" width=600 alt="Housing overview page">
<img src="demos/housing/reviews.jpg" width=600 alt="Housing reviews page">
<img src="demos/housing/write.jpg" width=600 alt="Housing new review page">

#### Dark Mode

<img src="demos/housing/home_dark.jpg" width=600 alt="Housing home page dark mode">
<img src="demos/housing/search_dark.jpg" width=600 alt="Housing search page dark mode">
<img src="demos/housing/overview_dark.jpg" width=600 alt="Housing overview page dark mode">
<img src="demos/housing/reviews_dark.jpg" width=600 alt="Housing reviews page dark mode">
<img src="demos/housing/write_dark.jpg" width=600 alt="Housing new review page dark mode">

## Additional Demos

_I do not take credit for the dashboards used. They are open source dashboards modified for my use. Their sources are linked in each title._

### [Caddy Dashboard](https://caddy.community/t/monitoring-caddy-server-with-grafana-prometheus-loki-on-debian/23314)

![Caddy Dashboard Picture](https://dqah5woojdp50.cloudfront.net/optimized/2X/3/3f1dd613f94eddf4f0a5ba77e75aa7c73c75c1aa_2_1380x714.png)

### [Meilisearch Dashboard](https://github.com/meilisearch/meilisearch/blob/main/assets/grafana-dashboard.json)

<img src="demos/devops/meilisearch.jpg" width=600 alt="Meilisearch dashboard">

### [Redis Dashboard](https://grafana.com/grafana/dashboards/11835-redis-dashboard-for-prometheus-redis-exporter-helm-stable-redis-ha/)

![Redis Dashboard Picture](https://grafana.com/api/dashboards/11835/images/7655/image)

### [ScyllaDB Dashboard](https://github.com/scylladb/scylla-monitoring)

![ScyllaDB Dashboard Picture](https://www.scylladb.com/wp-content/uploads/Grafana-Dashboards-1.jpg)

## Local Reproduction

![In Development](https://img.shields.io/badge/status-in--development-FFA500)

_Local Reproduction steps are being adjusted to account for new CI/CD pipelines. In the meantime check out our live website at [https://pickone.cc](https://pickone.cc)._
