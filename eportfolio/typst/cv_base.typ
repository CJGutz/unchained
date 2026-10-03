#import "@preview/basic-resume:0.2.9": *


#let name = "Carl Gutzkow"
#let location = "Oslo, Norway"
//#let email = "cjgutzkow@gmail.com"
#let github = "github.com/cjgutz"
#let linkedin = "linkedin.com/in/carl-gutzkow"
//#let phone = "+1 (xxx) xxx-xxxx"
#let personal-site = "gutzkow.com"

#show: resume.with(
  author: name,
  // All the lines below are optional.
  // For example, if you want to to hide your phone number:
  // feel free to comment those lines out and they will not show.
  location: location,
  //email: email,
  github: github,
  linkedin: linkedin,
  //phone: phone,
  personal-site: personal-site,
  accent-color: "#264280",
  font: "New Computer Modern",
  paper: "a4",
  author-position: left,
  personal-info-position: left,
)

== How I Help Your Business
Hire me if you need a passionate and opiniated software developer eager to understand the domain and the customer's needs.
I combine fast minimal viable experiments, grounded decisions, cuirosity to understand depth, team work, and software development life cycles to deliver on promises that requires responsibility and communication.


/*
 * Lines that start with == are formatted into section headings
 * You can use the specific formatting functions if needed
 * The following formatting functions are listed below
 * #edu(dates: "", degree: "", gpa: "", institution: "", location: "", consistent: false)
 * #work(company: "", dates: "", location: "", title: "")
 * #project(dates: "", name: "", role: "", url: "")
 * certificates(name: "", issuer: "", url: "", date: "")
 * #extracurriculars(activity: "", dates: "")
 * There are also the following generic functions that don't apply any formatting
 * #generic-two-by-two(top-left: "", top-right: "", bottom-left: "", bottom-right: "")
 * #generic-one-by-two(left: "", right: "")
 */
== Education

#edu(
  institution: "TU Delft",
  location: "Delft, Netherlands",
  dates: dates-helper(start-date: "2024", end-date: "2026"),
  degree: "MSc in Computer Science",

  // Uncomment the line below if you want edu formatting to be consistent with everything else
  consistent: true
)
- GPA: 8.5\/10
- Topics: Programming Languages, Computer Graphics, Quantum Computing, Geodatabases,
- Master Thesis: Deadlock Handling Strategies on Quantum Network Nodes

#edu(
  institution: "NTNU",
  location: "Trondheim, Norway",
  dates: dates-helper(start-date: "2021", end-date: "2024"),
  degree: "BSc in Computer Engineering",

  // Uncomment the line below if you want edu formatting to be consistent with everything else
  consistent: true
)
- GPA: 4.76\/5
- Topics: Software Engineering and Software Development Life Cycles
- Bachelor Thesis: A Tool to Analyze Verifiable Credentials (eIDAS 2.0)

== Work Experience

#work(
  title: "Full-Stack Developer",
  location: "Oslo",
  company: "Telescope",
  dates: dates-helper(start-date: "June 2022", end-date: "Present"),
)

- Co-founder of Telescope building the backend foundation of the entire application still used to this day
- Built the product allowing the company to raise 37 Million NOK
- Specialized in automating GIS analyses, but responsibility over all parts of the stack

#work(
  title: "Student Representative for Software Engineering NTNU",
  location: "Trondheim",
  company: "NTNU",
  dates: dates-helper(start-date: "2023", end-date: "2024"),
)
- Student representative in the evaluation committee for my study programme, performed every 5 years

== Voluntary Experience

#work(
  title: "Developer, Lead Developer, Deputy Commander and Commander",
  location: "Trondheim",
  company: "Hackerspace NTNU",
  dates: dates-helper(start-date: "Jun 2021", end-date: "May 2024"),
)
- Grew the tech team from 3 to 10 engaged members contributing to our digital infrastructure
- Upgraded managed servers with backups, monitoring, security, physical move, and (unfinished) matrix communication.
//- Organized social events and workshops for the organization
- Strengthened Hackerspaces' position to keep funding for longer
- Lead an organization growing from 10 to 30+ members

#work(
  title: "Developer",
  location: "Trondheim",
  company: "TIHLDE Index",
  dates: dates-helper(start-date: "2021", end-date: "2022"),
)
- Worked in a software development team for the study's student organization.
- Took part in the Backend development of collectible website achievements, strikes for missing events, the gallery, a capture-the-flag event, and continuous bug fixes.

*Reference Groups* - Dialog between students and professors

For the courses: Software Development 2 with agile project, Network Programming, and Machine Learning

== References
_Contact info available on request_

*Gustav Haaland*
CEO and Co-founder of Telescope

*Stephanie Wehner* Antoni van Leeuwenhoek Professor \
Quantum Internet Division, TU Delft \
Supervisor for my MSc thesis

// *Grethe Sandstrak*
// Associate Professor \
// Department of Computer Science, NTNU



// == Projects

// #project(
//   name: "Unchained",
//   // Role is optional
//   role: "Creator",
//   // Dates is optional
//   dates: dates-helper(start-date: "Apr 2024", end-date: "Present"),
//   // URL is also optional
//   url: "https://github.com/cjgutz/unchained",
// )


// #certificates(
//   name: "OSCP",
//   issuer: "Offensive Security",
//   // url: "",
//   date: "Oct 2024",
// )

// == Skills
// - *Programming Languages*: JavaScript, Python, C/C++, HTML/CSS, Java, Bash, R, Flutter, Dart
// - *Technologies*: React, Astro, Svelte, Tailwind CSS, Git, UNIX, Docker, Caddy, NGINX, Google Cloud Platform
