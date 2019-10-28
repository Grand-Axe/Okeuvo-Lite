# OkeuvoLite ![logo](Images/logo.png?raw=true "logo")
DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS HEADER.

Copyright (c) 2019 Asame Imoni Obiomah. All rights reserved.

Artificial intelligence ethics is of existential importance.
The licensing model of OkeuvoLite enforces adherence to a strict ethical code.

The contents of this file are subject to the terms of both the GNU
General Public License Version 2 only ("GPL") and Inverse license (collectively, the "License").  You
may not use this file except in compliance with the License.  You can
obtain a copy of the License at LICENSE.txt.  See the License for the specific
language governing permissions and limitations under the License.

When distributing the software, include this License Header Notice in each
file and include the License file at LICENSE.txt.

[Intro](#intro)<br />
[Ethics](#ethics)<br />
[Embrace Revolution](#embrace-revolution)<br />
[Format](#format)<br />
[How to](#how-to)<br />
[Some Ideas](#some-ideas)<br />
[Licensing](#licensing)<br />
[Rationale](#rationale)<br />
[Improvements and What Next](#improvements-and-what-next)

# Intro
**Alpha version 19.07.23 (codename, Ama)**

#### What's in the tin?

1. *the meaning grid, a  novel 2D grid of concepts,*
2. *code to transform every word, concept or idea into a mathematical object that can be quantified,*
3. *an abandoned patent application, [A Method for Affect](#embrace-revolution) that outlines a general artificial intelligence algorithm.*
<hr />
OkeuvoLite is a system of encoding knowledge based on, the *meaning grid*, a novel 2D grid of meaning concepts; each coordinate being a single source of truth, so that the meaning grid can represent all meaning by simple self-referencing. Unlike machine learning  word embeddings, it is *not* dependent on large volumes of training data, consequently it inherently lacks prejudices. Another plus is that since it doesn't require energy guzzling training data, it has a comparatively miniscule footprint on the environment - a win for the climate.

Note that the words meaning and concept are used interchangeably.

The ability to accurately encode meaning is General AI's Holy Grail. OkeuvoLite achieves this by robustly representing any word as a globally unique 2D position vector and any idea or concept as either a globally unique universal hash or a globally unique triangle of vectors; regardless of variation, such as; choice and order of words, sentence length etc; ***most importantly***, the concept can be anything, a shape, a sentence, a smell ... anything. 

2D representation presents all sorts of exciting new possibilities; including naturally handling polysemy, moving AI towards becoming mathematically provable and presenting a pathway to Turing completeness for natural language - that is, the ability to compile natural language as a set of unambiguous computer instructions.

The image below illustrates where OkeuvoLite lies in the hierarchy of autonomy.

![Hierarchy of autonomy](Images/hierarchyOfAutonomy.png?raw=true)



OkeuvoLite is an essential part of the LushCoin ecosystem. LushCoin is an open source cryptocurrency in development, its value as a means of payment is determined by its ability to evaluate the potential contribution of  inventions or other unique ideas to human wellbeing (see the [LushCoin white paper)](https://github.com/Grand-Axe/LushCoin/raw/master/Docs/LushCoinWhitePaper.pdf). 

The level of development of OkeuvoLite is now adequate for use in LushCoins consensus system, since the consensus system involves human intervention (so long as every use is tagged with the appropriate OkeuvoLite version). However, it will form the basis of the analytical methods described in the [LushCoin white paper)](https://github.com/Grand-Axe/LushCoin/raw/master/Docs/LushCoinWhitePaper.pdf) where greater accuracy is required, so feedback on the meaning grid described below will be most welcome.

***Meaning grid***

The [meaning grid](src/meaningGrid.csv) is a continuous 2D space on which meanings are clustered by complexity in a taxonomy of composable functions. The meaning grid is somewhat stilted at the moment, buts that's only due to meagre computational resources, so no worries.

Complexity is the estimated measure of a meanings aggregation of diverse functions. 

Let x and y be the coordinates of a concept on the meaning grid, then complexity *c* is,

*c = (x &middot; y) / 2*

The key phrase above is *composable functions*. 

Vector length *L* is,

*L = &radic;(x<sup>2</sup> &middot; y<sup>2</sup>)*

Here's a simplified depiction on the meaning grid of the vector representation of the sentence,

*"Have a beautiful day"*



![Have a beautiful day](Images/path.png?raw=true)

Increase in complexity of meanings is in proportion to increase in x and y coordinates. A consequence of this (through composability) is that items with higher x or y coordinates cannot be used in the definitions of items with lower x or y coordinates.

Another useful consequence is a rule of thumb, according to which the degree to which functions are shared between two meanings is proportional to the area of the triangle they share; this is illustrated below for the two meanings that correspond to coordinates *(x<sub>1</sub>, y<sub>2</sub>)* and *(x<sub>2</sub>, y<sub>3</sub>)*. The triangle, *(o, x<sub>1</sub>, y<sub>1</sub>)* depicts their shared functions (where *o* is the *y* ordinate of the root element of the meaning grid).

Triangle *(o, x<sub>1</sub>, y<sub>1</sub>)* defines where both meanings can interact (in any of the functions of their properties whose coordinates are within the shared area) as well as the number of ways both meanings can interact (the total number of functions in the shared area). The closer to the origin a region is, the more general the functions it holds are and the more likely it is to be a shared area. 

![Shared properties](Images/sharedProperties.png?raw=true)

*This property of the meaning grid is striking when put in context of the current state of the intellectual property process in which the utility and scope of a patent can only be delineated through multi-million dollar litigation - one of the many reasons why a whopping 97% of inventions fail. Madness. Here, we can simply solve the problem with a little math, calculating shared areas. not with millions of dollars, but at close to zero cost and in close to instant time.*

For now the meaning grid is provided as is. Please read the contents of the "Disclosure" folder to understand the theory behind it.

Composability ensures that every concept has a global numerical complexity that is unique to it. Further, composability makes it possible for meanings that do not exist on the meaning grid to be reliably built from those that already do.

Further, composability is the property that provides us new ways of representing huge amounts of data in compact, efficient forms as hashes or vectors or shapes etc. For example, there is a toy function, *convex_hull_sets*, in lib.rs that will enable you (with some manual GUI work on your part) to visualise meanings as convex hull sets as below.

![Convex hull sets of an arbitrary meaning](Images/convexHullSets.png?raw=true)

Importantly, composability makes it possible to validate concept definitions by three simple simple inequality rules:

1. a given pithy definition of a concept *d*, made with words and metrics from the meaning grid is perfect if *d = c &middot; n*
   Where *c* is the complexity property of the concept in the meaning grid and *n* is the number of unique clausal subjects in the discourse.
2. a definition provided for a concept leans toward inadequate if *d < c &middot; n*.
3. a definition given for a concept contains frivolities if *d > c &middot; n*.

Please note that the rules above might be inaccurate in certain cases as the meaning grid is a work in progress.

Functionally similar concepts tend to be clustered closely to one another in the meaning grid regardless of part of speech, so that two vectors starting and ending at similar coordinates tend to have similar meanings. For instance, note the similarity of the x, y coordinates in the meaning grid extract of word forms for the adjective, noun, adverb and verb sensual senses of the word "beauty". The column, synsetId is the SQL format of the Wordnet 3.0 offset, it is explained in the [How to](#how-to) section.

| synsetId  | lemma         | part of speech | x              | y               | gloss                                                        |
| --------- | ------------- | -------------- | -------------- | --------------- | ------------------------------------------------------------ |
| 300218305 | beauteous     | adjective      | 114.0655223066 | 1253.920318242  | (poetic) beautiful, especially to the sight                  |
| 104685396 | beauteousness | noun           | 114.0678279881 | 1254.5502490104 | the quality of being good looking and attractive             |
| 300217728 | beautiful     | adjective      | 114.0655223066 | 1253.920318242  | delighting the senses or exciting intellectual or emotional admiration |
| 400242006 | beautifully   | adverb         | 114.0655223066 | 1253.920318242  | in a beautiful manner                                        |
| 200293141 | beautify      | verb           | 114.0655223066 | 1253.920318242  | make more beautiful                                          |
| 104683814 | beauty        | noun           | 114.0655223066 | 1253.920318242  | the qualities that give pleasure to the senses               |



Two sentences with identical meanings, but differing construction as applied to pick of function words, parts of speech etc. will reduce to the same vector representation on the meaning grid.



![Summary](Images/summary.png?raw=true)




### Embrace Revolution
This repository also contains the release of a description of the worlds first and only algorithm for [artificial general intelligence](https://en.wikipedia.org/wiki/Artificial_general_intelligence) (human-like AI). It lays out a solid theoretical basis for general AI and can be found in the ***"Disclosure"*** folder.

Artificial general intelligence gives machines the capability of consciousness and autonomous ideation. This includes the ability to; make judgments based on self interest, to feel sentiments, differentiate good from bad etc.

The release was once a patent application which gave a valuable protection period while OkeuvoLite was being developed.

The release removes a conflict LushCoin had with its open source orientation, as well as with the degree of trust and transparency that  [The Creed](https://github.com/Grand-Axe/The-Creed) demands..



# Ethics

------

**IMPORTANT NOTE:** Giving the public ownership stake in AI is the only way to build the trust and societal confidence required for safe, healthy deployment of AI, particularly general AI. Therefore, this repo follows (as well as demands) strict adherence to [The Creed](https://github.com/Grand-Axe/The-Creed) - an AI ethics standard that is both rigorous and open source (thus owned by the public).

------



# Format

The overall format of [meaningGrid.csv](src/meaningGrid.csv) is:

1. Licensing information
2. Empty  space
3. Line containing OkeuvoLite version
4. Empty space
5. Synset/coordinate entries 

The synset coordinate entries consist of tab separated columns. The **synsetId** column bears Wordnet 3.0 offsets in the SQL format for Wordnet synsetId's; there is also an **x** coordinate (double) column and a **y** coordinate (double) column.<br />
An example entry, the line for the synset "beautiful" (used in the last illustration) is is shown below:

| synsetId  | x               | y                |
| --------- | --------------- | ---------------- |
| 300217728 | 57.032761153277 | 1253.92031824201 |

The functions in utils.rs, pos_num_to_subtract and pos_letter, help in conversions to and from Wordnet offset and Wordnet SQL formats.

# How to

Every meaning can be split into a fixed set of parts:
1. Subject
2. Object
3. Where (non-temporal objects that are modified by prepositions)
4. When (temporal objects that are modified by prepositions)
5. Instance(s) (array of named instances that can be either the subject or object)

***Who-what vector***

This is illustrated below. The big white arrow between subject and object represents a vector between both which adds (or removes) one or more properties (or the consequences of application of one or more properties) of the object to (or from) the attributes of the subject. Lets call this the who-what vector.

![Meaning tensor](Images/tensor.png?raw=true)

***Unit tensor***

The drawing above is really of a tensor of rank 3 at least (depending on the number of named instances) , consisting of the who-what vector and its directions pointing to "Where", "When" and to the items in the "Instance array". We will now refer to it as the unit tensor.

***Context***

The who-what vector is pure, devoid of context. The where, when and named instance directions confer context on the unit tensor.

***Excitation magnitude, excitation coordinates and excitation vector***

The change of state of the subject in the who-what vector is quantified by adding the vectors that extend from the origin of the meaning grid to the coordinates of the object, subject and their modifiers; the magnitude of the resultant is referred to as the excitation magnitude of the subject, while the end coordinates of the resultant are referred to as the excitation coordinates of the subject.

The excitation vector is the vector between the subject and its excitation coordinates.

### Method

This subsection explains how a hash - the standard input to LushCoins consensus system - can be produced. It relies on *ranking* of concepts in a discourse by aggregation of the compexities of their interactions.

***Preliminary  steps***

The steps to create the input expected by this library are:

1. Parse the discourse and disambiguate its words to Wordnet 3.0 senses. 
2. Where a word sense does not exist on the meaning grid, it can be defined using word senses that do.  The definition must be succinct and must also be a hyponym of an existing sense which must also double as the subject. After adding all the vectors in the definition, the new senses coordinates and magnitude are the excitation coordinates and excitation magnitude. This step has to be done manually at the moment - I might provide a GUI tool in the future.
3. Carry out coreference resolution.
4. Get the x,y coordinates for each of the disambiguated senses from the meaning grid.
5. Discard orphaned clauses.
6. Extract unit tensors from the discourse.
7. Discard all but one of any duplicate unit tensors.

Despite steps 1 to 3 above, note that code that consumes this library shouldn't include parsers, but should take parsed tokens as input instead. Parsing is an outside concern.



***Hash and universal vector generation***

Hashing encodes the sum of excitation vectors as well as the root modified type (the hypernym of the discourse) into a consistent, fixed length format. Similar concepts will yield similar hashes.

In order to create a fixed character hash, the elements are rounded to the nearest integer, however, the exact values can be obtained from the database.<br />A 29 digit hash (in practice they are 299 digits long) would look something like this:

*1!09!0!36-2!09!0!36-#!##!#!##*

The hash consists of unit tensor records that are separated by a hyphen. Each record contains elements that are further separated by an exclamation mark. Each element is based on the angle (hereafter, apex angle) of the apex vertex of a triangle that is formed between the either the unmodified coordinate or the excited coordinate, the origin and the maximum x value. The angles marked beta in the image below are the apex angles.

![Hash method explanation](Images/hashExplanation.png?raw=true)

The elements of each pair are a boolean "lean direction" ((ratio of resultant to distance to maximum x) > 1) and an integer value for the apex angle. The first pair describes the discourse hypernym, while the second describes the result of the summation of excitation vectors.

The "#"'s are paddings for null elements inserted to pad the number of characters in the hash to 299 where the hash has too few records.

Many would notice that, instead of a pair of numbers in an x, y coordinate, the apex angle is a single number that can uniquely represent any point in the 2D meaning grid space (depending on where points of measurement on the x axis - e.g. between - max(x) and +max(x)). However, the direction indication is included for ease. It would be useful to hear from you if savings in hash size are more useful than ease. 

The function, ***encode_discourse* ** in *lib.rs* encodes the discourse (or communication) as a time/unit tensor graph and dumps it in a database.

The function, ***get_hash*** in *lib.rs* retrieves the encoding from the database and generates the hash.



# Some Ideas

A few of the novel and exciting purposes to which OkeuvoLite's hashes can be put are listed below.

Open source currently doesn't have a robust funding mechanism, even where new methods with far reaching impact have been invented. In almost all cases, finding a source of funding for an open source effort involves a loss of independence.

OkeuvoLite will enable pioneering open source efforts that feature novel ideas to create a hash of their method, which in turn will be used to generate funding and royalties through LushCoin (especially for systems like [Wikipedia](https://wikipedia.org) whose data will be used intensively by LushCoin).

Yet another idea is to replace training (where suitable) by writing AI models as vectors on the meaning grid space.

# Licensing
The best way for humanity to be protected from malicious use of AI is to have a system of structures that gives the public a stake and and the authority of ownership to direct the future of AI and its impacts on their lives. We are entering an era of increase in individual capability that is orders of magnitude over what existed before - with this in mind, OkeuvoLite is open source to to those who accept its ethical code and is out of bounds to those who don't.

The LushCoin ecosystem leans on the following to ensure that AI always delivers benefits that enhance human wellbeing:
1. the blockchain to ensure trust,
2. a human focused, highly restrictive, open source ethical code for AI, [The Creed](https://github.com/Grand-Axe/The-Creed),
3. a license that locks out those who wish to operate outside the LushCoin blockchain and The Creed; the [Inverse license](Inverse License.md),
4. a free license that ensures inventions and innovations are robustly traceable to their original authors, the GPL version 2.

OkeuvoLite meets the above by being part of the LushCoin ecosystem. The code in this repository is dual licensed. Simple interpretation, to avoid negative exploitation of AI in any form, what you do with AI must be public; this code is open source based on two conditions:

1. You must be developing for use in LushCoin or have all your transactions recorded on the LushCoin blockchain via the LushCoin network.
2. You must be bound by [The Creed](https://github.com/Grand-Axe/The-Creed).

<hr/>
# Rationale
This work is dedicated to my late father, who departed this Earth on 22nd June 2016 and to my mother.

My fathers burial led directly to both this project and LushCoin. I had not been to my home town since childhood, added to that, I have spent most of my adulthood in the UK. On getting to my home town, Otovwodo, Warri (in the oil producing Niger Delta - [one of the 10 most polluted places in the world](http://science.time.com/2013/11/04/urban-wastelands-the-worlds-10-most-polluted-places/slide/niger-river-delta-nigeria/)), I met a shocking state of cramped living spaces and decay. Despite the oil.

However, the children's eyes were bright with healthy curiosity. Despite the cards dealt them, the adults were vigorous. Here are people with the ability and energy to succeed, to lift themselves if given half a chance, yet who are actively denied the opportunity by a rotten system. The heart bled.

I wondered how I could help lift my people in a country where the value of its currency depends, not on the talents and efforts of its people (and Nigeria has a whopping 180 million), but entirely on the international price of crude oil.<br />Its constitution offers a vivid example of how to build an "anticountry", it hosts the record anti-Earth phenomenon of six mentions of the phrase "oil and gas"! This is in sharp contrast to the constitutions of leading countries which instead, extol the virtues of individual rights, just reward for talent and enterprise, liberty and the responsibilities of the nation to the citizen.
Oil fuels everything in Nigeria; from a patronage system which erupts into a free flowing fountain of corruption, to subjugation, oppression and spiralling decay (mental and physical). Fires burn day and night, destroying climate and citizen.

#### Dilemma

The dilemma was real. How to help people help themselves within the law? I was already writing an artificial general intelligence called [Okeuvo](http://www.mindmutiny.com) and had begun to think of alternate currencies around which to build a stable business framework for people to help themselves through AI. Bitcoin cropped up in my research, a path that naturally led to blockchain. <br />Blockchain is a truly impressive algorithm whose public, immutable transaction trail crushes the problems of corruption; it has built in trust and cuts out the middle man. <br />So the concept of LushCoin was born, an AI- blockchain driven cryptocurrency that both promotes wellbeing and rewards innovation. This repo, OkeuvoLite, is the hashing component of LushCoin, the meaning grid is extracted from Okeuvo.

Together LushCoin, OkeuvoLite, [Awesome City](https://github.com/Grand-Axe/Awesome-City) and The Creed form an open source, publicly owned framework for safe, democratic delivery of both artificial narrow intelligence and artificial general intelligence.
Diverse areas can be revolutionised by harnessing the LushCoin framework. Various problems areas can solved as well, such; as making innovation toward greener living and reversing climate change profitable; allowing those out of work to earn a wage whilst maintaining their dignity; funding public health systems; staving off migration flows while preserving dignity; empowering society; sending an artificial consciousness owned by all on Earth to Mars (for instance) and so on.

<sub>Warri no de kari las, kpatakpata na dro.</sub>

# Improvements and What's Next

Function words have not yet been added, they will come in the next version. Treatment of negations will also come in the next version.<br />The coverage of Universal dependency properties is minor and needs to be expanded to 100%.

Code contribution would be most appreciated, so feel free to fork the repo. Code is expected to be imperially buggy at this early stage, so help with squashing any uppity bugs is needed. Also, this project is my "hello world" in Rust, so Rustacean veterans, put away your weapons :).

