use std::{collections::HashMap, io, ops::Range};

use aoc2023::err::SolutionError;
use regex::Regex;

fn main() -> io::Result<()> {
    println!(
        "part 1: {}",
        part_one(INPUT).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
    );

    Ok(())
}

fn part_one(input: &str) -> Result<CategoryItemId, SolutionError> {
    let (seeds, db) = parse_input(input)?;
    let dests = find_destination_ids(&db, &"seed", &"location", &seeds[..]);
    dests.min().ok_or_else(|| SolutionError::NoAnswer)
}

fn find_destination_ids<'a>(
    db: &'a LocMapDb<'a>,
    source: &'a Category<'a>,
    destination: &'a Category<'a>,
    starting_ids: &'a [CategoryItemId],
) -> impl Iterator<Item = CategoryItemId> + 'a {
    starting_ids
        .iter()
        .filter_map(|id| traverse_to(db, source, destination, id))
}

fn traverse_to(
    db: &LocMapDb<'_>,
    source: &Category<'_>,
    destination: &Category<'_>,
    starting_id: &CategoryItemId,
) -> Option<CategoryItemId> {
    let mut current_source = *source;
    let mut current_id = *starting_id;
    while let Some(mapping) = db.lookup_mapping_by_source(dbg!(&current_source)) {
        dbg!(mapping.source);
        dbg!(mapping.destination);

        if let Some((dest, id)) = mapping.destination_for(dbg!(&current_id)) {
            if dest == *destination {
                return Some(id);
            }

            current_source = dbg!(dest);
            current_id = dbg!(id);
        } else {
            dbg!("RETURNING NONE");
            return None;
        }
    }

    None
}

type Category<'a> = &'a str;

type CategoryItemId = i64;

/// (source range, dest range)
type MappingEntry = (Range<CategoryItemId>, Range<CategoryItemId>);

#[derive(Debug)]
struct LocMapping<'a> {
    source: Category<'a>,
    destination: Category<'a>,
    mappings: Vec<MappingEntry>,
}

impl<'a> LocMapping<'a> {
    fn new(
        source: Category<'a>,
        destination: Category<'a>,
        mappings: impl IntoIterator<Item = MappingEntry>,
    ) -> Self {
        Self {
            source,
            destination,
            mappings: mappings.into_iter().collect(),
        }
    }

    /// Any source numbers that aren't mapped correspond to the same destination number.
    /// So, seed number 10 corresponds to soil number 10.
    fn destination_for(
        &self,
        source_id: &CategoryItemId,
    ) -> Option<(Category<'a>, CategoryItemId)> {
        self.mappings
            .iter()
            .find_map(|(source, dest)| {
                if source.contains(source_id) {
                    let offset = source_id - source.start;
                    Some((self.destination, dest.start + offset))
                } else {
                    None
                }
            })
            .or(Some((self.destination, *source_id)))
    }
}

#[derive(Debug)]
struct LocMapDb<'a> {
    mappings: Vec<LocMapping<'a>>,
    index_by_source: HashMap<Category<'a>, usize>,
}

impl<'a> LocMapDb<'a> {
    fn new(mappings: impl IntoIterator<Item = LocMapping<'a>>) -> Self {
        let (mappings, index_by_source) = mappings.into_iter().enumerate().fold(
            (vec![], HashMap::new()),
            |(mut ms, mut ibs), (idx, m)| {
                ms.push(m);
                let stored_m = ms.last().unwrap();
                ibs.insert(stored_m.source, idx);
                (ms, ibs)
            },
        );

        Self {
            mappings,
            index_by_source,
        }
    }

    fn lookup_mapping_by_source(&self, source: &Category<'a>) -> Option<&LocMapping<'a>> {
        self.index_by_source
            .get(source)
            .and_then(|idx| self.mappings.get(*idx))
    }
}

fn parse_input(input: &str) -> Result<(Vec<CategoryItemId>, LocMapDb), SolutionError> {
    let trimmed = input.trim();
    let db_parser = DbParser::new();
    let seeds_parser = SeedsParser::new();
    let parts: Vec<&str> = Regex::new(r"(?m)^$").unwrap().splitn(trimmed, 2).collect();
    let (seed_part, db_part) = parts
        .get(0)
        .zip(parts.get(1))
        .ok_or_else(|| SolutionError::ParseError("Malformed input", input.to_owned()))?;
    let seeds = seeds_parser.parse(seed_part)?;
    let db = db_parser.parse(db_part)?;

    Ok((seeds, db))
}

#[derive(Debug)]
struct SeedsParser {
    structure_re: Regex,
    seed_re: Regex,
}

impl SeedsParser {
    fn new() -> Self {
        Self {
            structure_re: Regex::new(r"^seeds: (?<seeds>.*)$").unwrap(),
            seed_re: Regex::new(r"(\d+)").unwrap(),
        }
    }

    fn parse<'s>(&self, input: &'s str) -> Result<Vec<CategoryItemId>, SolutionError> {
        let seeds_str = self
            .structure_re
            .captures(input.trim())
            .and_then(|c| c.name("seeds"))
            .map(|m| m.as_str())
            .ok_or_else(|| SolutionError::ParseError("Malformed seeds header", input.to_owned()))?;

        self.seed_re
            .find_iter(seeds_str)
            .map(|m| {
                str::parse(m.as_str()).map_err(|_| {
                    SolutionError::ParseError("Malformed number", m.as_str().to_owned())
                })
            })
            .collect()
    }
}

#[derive(Debug)]
struct DbParser(Regex, MappingParser);

impl DbParser {
    fn new() -> Self {
        Self(Regex::new(r"(?m)^$").unwrap(), MappingParser::new())
    }

    fn parse<'s>(&self, s: &'s str) -> Result<LocMapDb<'s>, SolutionError> {
        let mappings = self
            .0
            .split(s)
            .filter_map(|s| {
                let trimmed = s.trim();
                if trimmed.is_empty() {
                    None
                } else {
                    Some(trimmed)
                }
            })
            .try_fold(vec![], |mut ms, chunk| {
                ms.push(self.1.parse(chunk)?);
                Ok(ms)
            })?;

        Ok(LocMapDb::new(mappings))
    }
}

#[derive(Debug)]
struct MappingParser {
    header_re: Regex,
    mapping_re: Regex,
}

impl MappingParser {
    fn new() -> Self {
        Self {
            header_re: Regex::new(r"^(?<source>\w+)-to-(?<destination>\w+) map:$").unwrap(),
            mapping_re: Regex::new(
                r"^(?<dest_range_start>\d+) (?<source_range_start>\d+) (?<range_size>\d+)$",
            )
            .unwrap(),
        }
    }

    fn parse<'s>(&self, s: &'s str) -> Result<LocMapping<'s>, SolutionError> {
        let mut lines = s.lines();
        let header = lines
            .next()
            .ok_or_else(|| SolutionError::ParseError("Empty mapping", s.to_owned()))?;

        let (_, [source, destination]) = self
            .header_re
            .captures(header)
            .map(|c| c.extract())
            .ok_or_else(|| {
                SolutionError::ParseError("Malformed mapping header", header.to_owned())
            })?;

        // Each line within a map contains three numbers: the destination range start, the source range start, and the range length.
        let mappings = lines
            .map(|l| -> Result<MappingEntry, SolutionError> {
                let (_, [dest_range_start_str, source_range_start_str, range_size_str]) = self
                    .mapping_re
                    .captures(l)
                    .map(|c| c.extract())
                    .ok_or_else(|| {
                        SolutionError::ParseError("Mapping line malformed", l.to_owned())
                    })?;
                let source_start =
                    str::parse::<CategoryItemId>(source_range_start_str).map_err(|_| {
                        SolutionError::ParseError("Malformed source range start", s.to_owned())
                    })?;
                let dest_start =
                    str::parse::<CategoryItemId>(dest_range_start_str).map_err(|_| {
                        SolutionError::ParseError("Malformed source range start", s.to_owned())
                    })?;
                let range_size = str::parse::<CategoryItemId>(range_size_str).map_err(|_| {
                    SolutionError::ParseError("Malformed source range start", s.to_owned())
                })?;

                Ok((
                    source_start..(source_start + range_size),
                    dest_start..(dest_start + range_size),
                ))
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(LocMapping::new(source, destination, mappings))
    }
}

#[test]
fn example_one() {
    let input = r#"
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"#;

    assert_eq!(35, part_one(input).unwrap())
}

const INPUT: &'static str = r#"
seeds: 1514493331 295250933 3793791524 105394212 828589016 654882197 658370118 49359719 4055197159 59237418 314462259 268880047 2249227634 74967914 2370414906 38444198 3291001718 85800943 2102534948 5923540

seed-to-soil map:
3352941879 1247490906 129850502
1738919961 2189748071 56658550
1795578511 292133467 518088747
1519757661 1666834550 130335907
1650093568 133993362 88826393
2813914030 2262539545 40894545
2698412916 2661705133 115501114
2854808575 810222214 437268692
410530961 1545057218 121777332
0 242661076 10731898
532308293 2303434090 61476099
3292077267 1797170457 2282798
2313667258 0 133993362
2447660620 2410952837 250752296
3294360065 253392974 38740493
10731898 1377341408 167715810
1457582089 2364910189 46042648
593784392 3009289500 824592362
3333100558 222819755 19841321
1418376754 1799453255 39205335
1503624737 2246406621 16132924
178447708 2777206247 232083253
3482792381 1838658590 351089481

soil-to-fertilizer map:
3513795976 4258851234 36116062
3393453635 4148223693 110627541
3504081176 3494735450 6350258
2671742993 3596285367 235915393
991290653 256764866 25867175
2907658386 3330719253 68855819
3336496635 4091266693 56957000
3161141476 2536943456 80523019
1961696534 304660310 29551079
812996514 1560772632 178294139
1562163347 2321959023 78904062
31289107 1039870587 339682886
1520029818 1928417376 42133529
3510431434 3832200760 3364542
3549912038 3272523584 58195669
0 149071717 31289107
3608107707 3501085708 5732766
503523937 1739066771 189350605
3071674583 3506818474 89466893
3241664495 3996434553 94832140
1708133363 454333361 253563171
3813493721 3097084118 175439466
1991247613 1970550905 351408118
3613840473 2897430870 199653248
2536943456 2762631333 134799537
1641067409 244319429 12445437
3994753216 3913095788 83338765
2976514205 3399575072 95160378
1354883134 0 143118415
1349131883 2400863085 5751251
1498001549 282632041 22028269
2342655731 180360824 63958605
370971993 1379553473 126598642
4149802438 2617466475 145164858
1017157828 707896532 331974055
692874542 334211389 120121972
497570635 143118415 5953302
3988933187 3907275759 5820029
4078091981 3835565302 71710457
1653512846 1506152115 54620517

fertilizer-to-water map:
3053686523 2028998994 1241280773
1492748555 1562401968 269616514
554432178 1000324407 562077561
2944878746 3270279767 108807777
2501520804 3379087544 52288887
1762365069 20686928 69653413
2028998994 3431376431 472521810
1431309984 814973200 3653900
409832614 0 20686928
430519542 818627100 123912636
1116509739 500172955 314800245
2553809691 3903898241 391069055
0 90340341 409832614
1434963884 942539736 57784671

water-to-light map:
2774754469 1598606098 15160294
3832622498 1469118874 129487224
4125818569 3997047227 169148727
1108418694 1130695768 196125912
637654660 517892123 26551592
2519230072 3399515763 135968347
2751000257 3126996880 23754212
861800165 884077239 88532605
1469118874 2566660966 63929427
2789914763 3150751092 63425583
2853340346 2561960449 4700517
1692681911 3397592997 1922766
2858040863 1773257241 287341147
2655198419 2851614649 15098309
950332770 972609844 158085924
591049922 471287385 46604738
3482333989 3849276196 108901632
1694604677 1613766392 159490849
3591235621 3958177828 38869399
3630105020 4166195954 128771342
3205039781 3535484110 277294208
2706794606 2060598388 44205651
386607659 544443715 110595292
3962109722 2540862463 21097986
697311924 0 92263281
558450415 404456802 11447837
2409200012 3214176675 110030060
569898252 862925569 21151670
1354525884 415904639 55382746
789575205 790700609 72224960
3832262624 2690248164 359874
163757080 181606223 222850579
1533048301 2967363270 159633610
497202951 784452458 6248151
2670296728 3812778318 36497878
3145382010 2630590393 59657771
1854095526 2104804039 436058424
0 655039007 74414138
2390804262 2690608038 18395750
1304544606 1326821680 49981278
3758876362 3324206735 73386262
664206252 1376802958 33105672
74414138 92263281 89342942
3983207708 2709003788 142610861
503451102 729453145 54999313
2290153950 2866712958 100650312

light-to-temperature map:
2054128675 422374783 216418447
3729049939 3132111492 565917357
524183620 1261361039 34450583
723901655 638793230 74616934
304496246 795175951 115896188
1128506994 1008723417 50927515
958650763 2074746732 3056214
0 1224918384 36442655
558634203 1059650932 165267452
2052385426 713410164 1743249
833053864 1953862956 120883776
36442655 715153413 80022538
1455705971 1506174459 386316618
1842022589 1295811622 210362837
2270547122 2077802946 24477429
116465193 2102280375 188031053
1179434509 945607414 63116003
798518589 911072139 34535275
3373647615 3893335786 195498774
3696979816 3698028849 32070123
1242550512 1892491077 43410067
3051130577 2566857633 322517038
3002478931 3844684140 48651646
2311039190 3730098972 114585168
2425624358 2311039190 187630064
2819387158 2889374671 183091773
1285960579 103791186 85466201
1371426780 338095592 84279191
1110545182 1935901144 17961812
953937640 2290311428 4713123
420392434 0 103791186
961706977 189257387 148838205
3637334768 3072466444 59645048
3569146389 2498669254 68188379
2613254422 4088834560 206132736

temperature-to-humidity map:
2032423062 2486277941 26281270
333062067 2316624216 6051173
1716048249 1385455997 91409968
460397469 2512559211 69041956
25538975 668468772 15388105
2090913379 324344034 71221218
2804805674 3018690414 130533369
529439425 908821722 476634275
1106765273 448560048 219908724
2162134597 1476865965 445565302
0 395565252 25538975
4004204691 4019550481 43244121
2058704332 0 32209047
1516375281 2116951248 199672968
3888833667 2764269134 115371024
40927080 32209047 292134987
1490276549 2581601167 26098732
1360818538 2322675389 129458011
1006073700 2043715496 73235752
1079309452 421104227 27455821
1807458217 683856877 224964845
339113240 1922431267 121284229
4155917040 2879640158 139050256
2764269134 4171262830 40536540
4047448812 4062794602 108468228
2935339043 3149223783 870326698
1326673997 2452133400 34144541
3805665741 4211799370 83167926

humidity-to-location map:
3928575650 3147563455 98804874
2357899446 2418187254 26586982
449562184 2261875136 59054833
308294839 60287808 141267345
1872062279 1591999301 10857495
1227084719 574109504 259895254
1980177059 1765508840 13399403
1162615704 1941046629 64469015
2384486428 2654234915 114059261
702311863 1515542756 40659242
1993576462 2005515644 256359492
3047301127 3246368329 27789688
650396000 1713592977 51915863
3701335066 3862068622 181049310
1122863338 1673840611 39752366
3075090815 4174917460 120049836
3882384376 2778717800 46191274
1626219043 201555153 245843236
4027380524 3325898523 267586772
3417979817 4043117932 13866284
3281454033 2925002624 18592540
110359150 1556201998 35797303
2558833497 2444774236 209460679
508617017 834004758 141778983
1574315753 975783741 51903290
2249935954 1027687031 107963492
0 463750354 110359150
2778717800 3593485295 268583327
3195140651 2824909074 86313382
3445626269 2943595164 203968291
3324037823 4080975466 93941994
742971105 1135650523 379892233
146156453 1778908243 39038597
3300046573 4056984216 23991250
1503331938 1602856796 70983815
185195050 1817946840 123099789
3649594560 3274158017 21920411
1486979973 447398389 16351965
3671514971 3296078428 29820095
2498545689 0 60287808
3431846101 2911222456 13780168
1882919774 2320929969 97257285
"#;
