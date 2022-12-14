<!-- Improved compatibility of back to top link: See: https://github.com/othneildrew/Best-README-Template/pull/73 -->
<a name="readme-top"></a>
<!--
*** Thanks for checking out the Best-README-Template. If you have a suggestion
*** that would make this better, please fork the repo and create a pull request
*** or simply open an issue with the tag "enhancement".
*** Don't forget to give the project a star!
*** Thanks again! Now go create something AMAZING! :D
-->



<!-- PROJECT SHIELDS -->
<!--
*** I'm using markdown "reference style" links for readability.
*** Reference links are enclosed in brackets [ ] instead of parentheses ( ).
*** See the bottom of this document for the declaration of the reference variables
*** for contributors-url, forks-url, etc. This is an optional, concise syntax you may use.
*** https://www.markdownguide.org/basic-syntax/#reference-style-links
-->
[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![AGPLv3 License][license-shield]][license-url]
[![LinkedIn][linkedin-shield]][linkedin-url]

<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/thenulldev/api">
    <img src="logo.png" alt="Logo" width="80" height="80">
  </a>

<h3 align="center">Null API</h3>

  <p align="center">
    API For collecting data
    <br />
    <a href="https://github.com/thenulldev/api"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://github.com/thenulldev/api">View Demo</a>
    ·
    <a href="https://github.com/thenulldev/api/issues">Report Bug</a>
    ·
    <a href="https://github.com/thenulldev/api/issues">Request Feature</a>
  </p>
</div>

<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
        <li><a href="#benchmark">Benchmark</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
  </ol>
</details>

<!-- ABOUT THE PROJECT -->
## About The Project

This is a project to track stats on services I use

<p align="right">(<a href="#readme-top">back to top</a>)</p>

### Built With

* Actix-Web
* SQLx
* Redis
* Reqwest

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- GETTING STARTED -->
## Getting Started

This is an example of how you can get started contributing.

### Prerequisites

This is an example of how to list things you need to use the software and how to install them.

* Postgres
* Redis
* Duolingo [API](#) Key
* Spotify [API](#) Key
* Github [API](#) Key

### Installation

1. Clone the repo
   ```sh
   git clone https://github.com/thenulldev/api.git && cd api
   ```
2. Copy the `.env.example`
   ```sh
   cp .env.example .env
   ```
3. Run the API
   ```sh
   cargo run
   ```
## Benchmark



1. Install [Drill](https://github.com/fcsonline/drill) benchmarking suite
   ```sh
   cargo install drill
   ```
2. Run the API
   ```sh
   cargo run
3. Open a new terminal and run the benchmark
   ```sh
   drill --benchmark benchmark.yml --stats
   ```
<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- USAGE EXAMPLES -->
## Usage

This is mainly used to track my personal stats to display on a dashboard and for learning.


<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- ROADMAP -->
## Roadmap

- [x] Spotify Stats
- [x] Duolingo Stats
- [x] Github Stats
- [ ] Waka Stats

See the [open issues](https://github.com/thenulldev/api/issues) for a full list of proposed features (and known issues).

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- CONTRIBUTING -->
## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- LICENSE -->
## License

Distributed under the MIT License. See `LICENSE.txt` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- CONTACT -->
## Contact

Stephen Freerking - [@SnipeyDev](https://twitter.com/SnipeyDev) - stephen@thenull.dev

Project Link: [https://github.com/thenulldev/api](https://github.com/thenulldev/api)

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/thenulldev/api.svg?style=for-the-badge
[contributors-url]: https://github.com/thenulldev/api/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/thenulldev/api.svg?style=for-the-badge
[forks-url]: https://github.com/thenulldev/api/network/members
[stars-shield]: https://img.shields.io/github/stars/thenulldev/api.svg?style=for-the-badge
[stars-url]: https://github.com/thenulldev/api/stargazers
[issues-shield]: https://img.shields.io/github/issues/thenulldev/api.svg?style=for-the-badge
[issues-url]: https://github.com/thenulldev/api/issues
[license-shield]: https://img.shields.io/github/license/thenulldev/api.svg?style=for-the-badge
[license-url]: https://github.com/thenulldev/api/blob/master/LICENSE.txt
[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=for-the-badge&logo=linkedin&colorB=555
[linkedin-url]: https://linkedin.com/in/stephenfdev