use std::collections::HashMap;
use std::rc::Rc;
use std::mem::replace;
fn main() {
    let data0 = "CD Title,CD ID,CD Label,Performer,Orchestra,Conductor,Time
Brandenburg Concertos ##1-3,OQ0062,Concerto Classics,,Hamburg Symphony Orchestra,Gabor Otücs,
Brandenburg Concertos ##4-6,OQ0063,Concerto Classics,,Hamburg Symphony Orchestra,Gabor Otücs,
Christmas Oratoria,8.550428-430,Naxos,Various,Failoni Radio Orch./Hungarian Radio Choir,Géza Oberfrank,148.06
Harpsichord Concertos,PCD 864,Pickwick,Ivor Bolton,St. James's Baroque Players,Ivor Bolton,63.18
\"Cantatas (\"\"Coffee\"\"/\"\"Peasant\"\")\",8.550641,Naxos,Various,Failoni Chamber Orchestra,Mátyás Antál,51.42
Cantatas BWV 80 & BWV 147,8.550642,Naxos,Various,Failoni Chamber Orchestra,Mátyás Antál,53.48
Bach ViolinConcertos,CDA66380,Hyperion,Catherine MacKintosh,The King's Consort,Robert King,58.5
Piano Concertos (Complete) Vol 2,8.550423,Naxos,Hae-won Chang,Camerata Cassovia,Robert Stankovsky,57.12
Piano Concertos (Complete) Vol 1,8.550422,Naxos,Hae-won Chang,Camerata Cassovia,Robert Stankovsky,61.55
Goldberg Variations,SMN 52594,Sony,Glenn Gould,,,46.11
Two and Three Part Inventions,SMK 52596,Sony,Glenn Gould,,,50.04
The Well Tempered Clavier Part 1 ##13-24,CLS 4205,Masters Classic,Christiane Jaccottet,,,50.49
The Well Tempered Clavier Part 1 ##1-12,CLS 4203,Masters Classic,Christiane Jaccottet,,,51.28
Handel / Bach Harpsichord,SMN 52590,Sony,Glenn Gould,,,55.11
French Suites ##1 & 2,8.550709,Naxos,Wolfgang Rübsam,,,55.24
Inventions & Sinfoniae,1905161,Harmonia Mundi,Tatiana Nikolaeva,,,56.11
J.S. Bach Piano Works,8.550571,Naxos,János Sebestyén,,,63.16
French Suites ##3 - 6,8.550710,Naxos,Wolfgang Rübsam,,,65.46
Sonatas and Partitas for Solo Violin Vol 2,8.550570,Naxos,Christiane Edinger,,,76.12
Sonatas and Partitas for Solo Violin Vol 1,8.550569,Naxos,Christiane Edinger,,,77
The Well Tempered Clavier Book 1,SMN 52600,Sony,Glenn Gould,,,106.39
Die Kunst der Fugue,CDA66631/2,Hyperion,Tatiana Nikolaeva,,,122.15
Sechs Suiten für Violincello solo,419 359-2,DG,Pierre Fournier,,,138.45
Goldberg Variationen,WVH 032,Arts Music,Pieter Jan Belder,,,";

    let data1 = "Composition Name,BWV Num,CD Label,CD ID,Instrument,key major/minor,key
Invention C Major,BWV 772,Harmonia Mundi,1905161,Piano,C Major,C
Invention C Minor,BWV 773,Harmonia Mundi,1905161,Piano,C Minor,C
Invention D Major,BWV 774,Harmonia Mundi,1905161,Piano,D Major,D
Invention D Minor,BWV 775,Harmonia Mundi,1905161,Piano,D Minor,D
Invention Eb Major,BWV 776,Harmonia Mundi,1905161,Piano,Eb Major,Eb
Invention E Major,BWV 777,Harmonia Mundi,1905161,Piano,E Major,E
Invention E Minor,BWV 778,Harmonia Mundi,1905161,Piano,E Minor,E
Invention F Major,BWV 779,Harmonia Mundi,1905161,Piano,F Major,F
Invention F Minor,BWV 780,Harmonia Mundi,1905161,Piano,F Minor,F
Invention G Major,BWV 781,Harmonia Mundi,1905161,Piano,G Major,G
Invention G Minor,BWV 782,Harmonia Mundi,1905161,Piano,G Minor,G
Invention A Major,BWV 783,Harmonia Mundi,1905161,Piano,A Major,A
Invention A Minor,BWV 784,Harmonia Mundi,1905161,Piano,A Minor,A
Invention Bb Major,BWV 785,Harmonia Mundi,1905161,Piano,Bb Major,Bb
Invention B Minor,BWV 786,Harmonia Mundi,1905161,Piano,B Minor,B
Symphoniae C Major,BWV 787,Harmonia Mundi,1905161,Piano,C Major,C
Symphoniae C Minor,BWV 788,Harmonia Mundi,1905161,Piano,C Minor,C
Symphoniae D Major,BWV 789,Harmonia Mundi,1905161,Piano,D Major,D
Symphoniae D Minor,BWV 790,Harmonia Mundi,1905161,Piano,D Minor,D
Symphoniae Eb Major,BWV 791,Harmonia Mundi,1905161,Piano,Eb Major,Eb
Symphoniae E Major,BWV 792,Harmonia Mundi,1905161,Piano,E Major,E
Symphoniae E Minor,BWV 793,Harmonia Mundi,1905161,Piano,E Minor,E
Symphoniae F Major,BWV 794,Harmonia Mundi,1905161,Piano,F Major,F
Symphoniae F Minor,BWV 795,Harmonia Mundi,1905161,Piano,F Minor,F
Symphoniae G Major,BWV 796,Harmonia Mundi,1905161,Piano,G Major,G
Symphoniae G Minor,BWV 797,Harmonia Mundi,1905161,Piano,G Minor,G
Symphoniae A Major,BWV 798,Harmonia Mundi,1905161,Piano,A Major,A
Symphoniae A Minor,BWV 799,Harmonia Mundi,1905161,Piano,A Minor,A
Symphoniae Bb Major,BWV 800,Harmonia Mundi,1905161,Piano,Bb Major,Bb
Symphoniae B Minor,BWV 801,Harmonia Mundi,1905161,Piano,B Minor,B
Suite #3 C Major,BWV 1009,DG,419 359-2,Violincello,C Major,C
Suite #1 G Major,BWV 1007,DG,419 359-2,Violincello,G Major,G
Suite #5 C Minor,BWV 1011,DG,419 359-2,Violincello,C Minor,C
Suite #2 D Minor,BWV 1008,DG,419 359-2,Violincello,D Minor,D
Suite #4 Eb Major,BWV 1010,DG,419 359-2,Violincello,Eb Major,Eb
Suite #6 D Major,BWV 1012,DG,419 359-2,Violincello,D Major,D
Concerto in D Minor,BWV 1052,Naxos,8.550422,Piano,D Minor,D
Concerto in E Major,BWV 1053,Naxos,8.550422,Piano,E Major,E
Concerto in D Major,BWV 1054,Naxos,8.550422,Piano,D Major,D
Concerto in A Major,BWV 1055,Naxos,8.550423,Piano,A Major,A
Concerto in F Minor,BWV 1056,Naxos,8.550423,Piano,F Minor,F
Concerto in F Major,BWV 1057,Naxos,8.550423,Piano,F Major,F
Concerto in G Minor,BWV 1058,Naxos,8.550423,Piano,G Minor,G
Christmas Oratorio,BWV 248,Naxos,8.550428-430,Orchestra/Choir,,
Sonata #1 G Minor,BWV 1001,Naxos,8.550569,Violin,G Minor,G
Partita #1 B Minor,BWV 1002,Naxos,8.550569,Violin,B Minor,B
Sonata #2 A Minor,BWV 1003,Naxos,8.550569,Violin,A Minor,A
Partita #2 D Minor,BWV 1004,Naxos,8.550570,Violin,D Minor,D
Sonata #3 C Major,BWV 1005,Naxos,8.550570,Violin,C Major,C
Partita #3 E Major,BWV 1006,Naxos,8.550570,Violin,E Major,E
Italian Concerto,BWV 971,Naxos,8.550571,Piano,,
Fantasia & Fugue A Minor,BWV 904,Naxos,8.550571,Piano,A Minor,A
Praeambulum C Major,BWV 924,Naxos,8.550571,Piano,C Major,C
Prelude C Major,BWV 939,Naxos,8.550571,Piano,C Major,C
Prelude C Minor,BWV 999,Naxos,8.550571,Piano,C Minor,C
Prelude D Major,BWV 925,Naxos,8.550571,Piano,D Major,D
Prelude D Minor,BWV 926,Naxos,8.550571,Piano,D Minor,D
Prelude D Minor,BWV 940,Naxos,8.550571,Piano,D Minor,D
Prelude E Minor,BWV 941,Naxos,8.550571,Piano,E Minor,E
Praeambulum F Major,BWV 927,Naxos,8.550571,Piano,F Major,F
Prelude F Major,BWV 928,Naxos,8.550571,Piano,F Major,F
Trio G Minor,BWV 929,Naxos,8.550571,Piano,G Minor,G
Praeambulum G Minor,BWV 930,Naxos,8.550571,Piano,G Minor,G
Prelude A Minor,BWV 942,Naxos,8.550571,Piano,A Minor,A
Two Part Invention F Major,BWV 779,Naxos,8.550571,Piano,F Major,F
Fantasia C Minor,BWV 906,Naxos,8.550571,Piano,C Minor,C
Prelude Fugue & Allegro Eb Major,BWV 998,Naxos,8.550571,Piano,Eb Major,Eb
Chromatic Fantasia & Fugue D Minor,BWV 903,Naxos,8.550571,Piano,D Minor,D
Coffee Cantata,BWV 211,Naxos,8.550641,Orchestra/Vocal,,
Peasant Cantata,BWV 212,Naxos,8.550641,Orchestra/Vocal,,
Ein feste Burg ist unser Gott,BWV 80,Naxos,8.550642,Orchestra/Vocal,,
Herz und Mund und Tat und Leben,BWV 147,Naxos,8.550642,Orchestra/Vocal,,
Italian Concerto,BWV 971,Naxos,8.550709,Piano,,
Chromatic Fantasia & Fugue D Minor,BWV 903,Naxos,8.550709,Piano,D Minor,D
French Suite #1 D Minor,BWV 812,Naxos,8.550709,Piano,D Minor,D
French Suite #2 C Minor,BWV 813,Naxos,8.550709,Piano,C Minor,C
French Suite #3 B Minor,BWV 814,Naxos,8.550710,Piano,B Minor,B
French Suite #4 Eb Minor,BWV 815,Naxos,8.550710,Piano,Eb Minor,Eb
French Suite #5 G Major,BWV 816,Naxos,8.550710,Piano,G Major,G
French Suite #6 E Major,BWV 817,Naxos,8.550710,Piano,E Major,E
Violin Concerto D Minor,BWV 1043,Hyperion,CDA66380,Violin,D Minor,D
Violin Concerto E Major,BWV 1042,Hyperion,CDA66380,Violin,E Major,E
Violin Concerto A Minor,BWV 1041,Hyperion,CDA66380,Violin,A Minor,A
Violin/Oboe Concerto C Minor,BWV 1060,Hyperion,CDA66380,Violin/Oboe,C Minor,C
Two Ricercari,BWV 1079,Hyperion,CDA66631/2,Piano,,
Duet #1 E Minor,BWV 802,Hyperion,CDA66631/2,Piano,E Minor,E
Duet #2 F Major,BWV 803,Hyperion,CDA66631/2,Piano,F Major,F
Duet #3 G Major,BWV 804,Hyperion,CDA66631/2,Piano,G Major,G
Duet #4 A Minor,BWV 805,Hyperion,CDA66631/2,Piano,A Minor,A
Die Kunst der Fugue,BWV 1080,Hyperion,CDA66631/2,Piano,,
Prelude & Fugue #1 C Major,BWV 846,Masters Classics,CLS 4203,Harpsichord,C Major,C
Prelude & Fugue #2 C Minor,BWV 847,Masters Classics,CLS 4203,Harpsichord,C Minor,C
Prelude & Fugue #3 C# Major,BWV 848,Masters Classics,CLS 4203,Harpsichord,C# Major,C#
Prelude & Fugue #4 C# Minor,BWV 849,Masters Classics,CLS 4203,Harpsichord,C# Minor,C#
Prelude & Fugue #5 D Major,BWV 850,Masters Classics,CLS 4203,Harpsichord,D Major,D
Prelude & Fugue #6 D Minor,BWV 851,Masters Classics,CLS 4203,Harpsichord,D Minor,D
Prelude & Fugue #7 Eb Major,BWV 852,Masters Classics,CLS 4203,Harpsichord,Eb Major,Eb
Prelude & Fugue #8 Eb Minor,BWV 853,Masters Classics,CLS 4203,Harpsichord,Eb Minor,Eb
Prelude & Fugue #9 E Major,BWV 854,Masters Classics,CLS 4203,Harpsichord,E Major,E
Prelude & Fugue #10 E Minor,BWV 855,Masters Classics,CLS 4203,Harpsichord,E Minor,E
Prelude & Fugue #11 F Major,BWV 856,Masters Classics,CLS 4203,Harpsichord,F Major,F
Prelude & Fugue #12 F Minor,BWV 857,Masters Classics,CLS 4203,Harpsichord,F Minor,F
Prelude & Fugue #13 F# Major,BWV 858,Masters Classics,CLS 4205,Harpsichord,F# Major,F#
Prelude & Fugue #14 F# Minor,BWV 859,Masters Classics,CLS 4205,Harpsichord,F# Minor,F#
Prelude & Fugue #15 G Major,BWV 860,Masters Classics,CLS 4205,Harpsichord,G Major,G
Prelude & Fugue #16 G Minor,BWV 861,Masters Classics,CLS 4205,Harpsichord,G Minor,G
Prelude & Fugue #17 Ab Major,BWV 862,Masters Classics,CLS 4205,Harpsichord,Ab Major,Ab
Prelude & Fugue #18 G# Minor,BWV 863,Masters Classics,CLS 4205,Harpsichord,G# Minor,G#
Prelude & Fugue #19 A Major,BWV 864,Masters Classics,CLS 4205,Harpsichord,A Major,A
Prelude & Fugue #20 A Minor,BWV 865,Masters Classics,CLS 4205,Harpsichord,A Minor,A
Prelude & Fugue #21 Bb Major,BWV 866,Masters Classics,CLS 4205,Harpsichord,Bb Major,Bb
Prelude & Fugue #22 Bb Minor,BWV 867,Masters Classics,CLS 4205,Harpsichord,Bb Minor,Bb
Prelude & Fugue #23 B Major,BWV 868,Masters Classics,CLS 4205,Harpsichord,B Major,B
Prelude & Fugue #24 B Minor,BWV 869,Masters Classics,CLS 4205,Harpsichord,B Minor,B
Brandenburg Concerto #1 F Major,BWV 1046,Concerto Classics,OQ0062,Orchestra,F Major,F
Brandenburg Concerto #2 F Major,BWV 1047,Concerto Classics,OQ0062,Orchestra,F Major,F
Brandenburg Concerto #3 G Major,BWV 1048,Concerto Classics,OQ0062,Orchestra,G Major,G
Brandenburg Concerto #4 G Major,BWV 1049,Concerto Classics,OQ0063,Orchestra,G Major,G
Brandenburg Concerto #5 D Major,BWV 1050,Concerto Classics,OQ0063,Orchestra,D Major,D
Brandenburg Concerto #6 Bb Major,BWV 1051,Concerto Classics,OQ0063,Orchestra,Bb Major,Bb
Concerto in D Minor,BWV 1052,Pickwick,PCD 864,Harpsichord,D Minor,D
Concerto in A Major,BWV 1055,Pickwick,PCD 864,Harpsichord,A Major,A
Concerto in F Minor,BWV 1056,Pickwick,PCD 864,Harpsichord,F Minor,F
Concerto in F Major,BWV 1057,Pickwick,PCD 864,Harpsichord,F Major,F
Invention C Major,BWV 772,Sony,SMK 52596,Piano,C Major,C
Invention C Minor,BWV 773,Sony,SMK 52596,Piano,C Minor,C
Invention D Major,BWV 774,Sony,SMK 52596,Piano,D Major,D
Invention D Minor,BWV 775,Sony,SMK 52596,Piano,D Minor,D
Invention Eb Major,BWV 776,Sony,SMK 52596,Piano,Eb Major,Eb
Invention E Major,BWV 777,Sony,SMK 52596,Piano,E Major,E
Invention E Minor,BWV 778,Sony,SMK 52596,Piano,E Minor,E
Invention F Major,BWV 779,Sony,SMK 52596,Piano,F Major,F
Invention F Minor,BWV 780,Sony,SMK 52596,Piano,F Minor,F
Invention G Major,BWV 781,Sony,SMK 52596,Piano,G Major,G
Invention G Minor,BWV 782,Sony,SMK 52596,Piano,G Minor,G
Invention A Major,BWV 783,Sony,SMK 52596,Piano,A Major,A
Invention A Minor,BWV 784,Sony,SMK 52596,Piano,A Minor,A
Invention Bb Major,BWV 785,Sony,SMK 52596,Piano,Bb Major,Bb
Invention B Minor,BWV 786,Sony,SMK 52596,Piano,B Minor,B
Symphoniae C Major,BWV 787,Sony,SMK 52596,Piano,C Major,C
Symphoniae C Minor,BWV 788,Sony,SMK 52596,Piano,C Minor,C
Symphoniae D Major,BWV 789,Sony,SMK 52596,Piano,D Major,D
Symphoniae D Minor,BWV 790,Sony,SMK 52596,Piano,D Minor,D
Symphoniae Eb Major,BWV 791,Sony,SMK 52596,Piano,Eb Major,Eb
Symphoniae E Major,BWV 792,Sony,SMK 52596,Piano,E Major,E
Symphoniae E Minor,BWV 793,Sony,SMK 52596,Piano,E Minor,E
Symphoniae F Major,BWV 794,Sony,SMK 52596,Piano,F Major,F
Symphoniae F Minor,BWV 795,Sony,SMK 52596,Piano,F Minor,F
Symphoniae G Major,BWV 796,Sony,SMK 52596,Piano,G Major,G
Symphoniae G Minor,BWV 797,Sony,SMK 52596,Piano,G Minor,G
Symphoniae A Major,BWV 798,Sony,SMK 52596,Piano,A Major,A
Symphoniae A Minor,BWV 799,Sony,SMK 52596,Piano,A Minor,A
Symphoniae Bb Major,BWV 800,Sony,SMK 52596,Piano,Bb Major,Bb
Symphoniae B Minor,BWV 801,Sony,SMK 52596,Piano,B Minor,B
Prelude & Fugue E Major,BWV 878,Sony,SMN 52590,Piano,E Major,E
Prelude & Fugue F# Minor,BWV 883,Sony,SMN 52590,Piano,F# Minor,F#
Goldberg Variations,BWV 988,Sony,SMN 52594,Piano,,
Fugue F# Minor,BWV 883,Sony,SMN 52594,Piano,F# Minor,F#
Fugue E Major,BWV 878,Sony,SMN 52594,Piano,E Major,E
Prelude & Fugue #1 C Major,BWV 846,Sony,SMN 52600,Piano,C Major,C
Prelude & Fugue #2 C Minor,BWV 847,Sony,SMN 52600,Piano,C Minor,C
Prelude & Fugue #3 C# Major,BWV 848,Sony,SMN 52600,Piano,C# Major,C#
Prelude & Fugue #4 C# Minor,BWV 849,Sony,SMN 52600,Piano,C# Minor,C#
Prelude & Fugue #5 D Major,BWV 850,Sony,SMN 52600,Piano,D Major,D
Prelude & Fugue #6 D Minor,BWV 851,Sony,SMN 52600,Piano,D Minor,D
Prelude & Fugue #7 Eb Major,BWV 852,Sony,SMN 52600,Piano,Eb Major,Eb
Prelude & Fugue #8 Eb Minor,BWV 853,Sony,SMN 52600,Piano,Eb Minor,Eb
Prelude & Fugue #9 E Major,BWV 854,Sony,SMN 52600,Piano,E Major,E
Prelude & Fugue #10 E Minor,BWV 855,Sony,SMN 52600,Piano,E Minor,E
Prelude & Fugue #11 F Major,BWV 856,Sony,SMN 52600,Piano,F Major,F
Prelude & Fugue #12 F Minor,BWV 857,Sony,SMN 52600,Piano,F Minor,F
Prelude & Fugue #13 F# Major,BWV 858,Sony,SMN 52600,Piano,F# Major,F#
Prelude & Fugue #14 F# Minor,BWV 859,Sony,SMN 52600,Piano,F# Minor,F#
Prelude & Fugue #15 G Major,BWV 860,Sony,SMN 52600,Piano,G Major,G
Prelude & Fugue #16 G Minor,BWV 861,Sony,SMN 52600,Piano,G Minor,G
Prelude & Fugue #17 Ab Major,BWV 862,Sony,SMN 52600,Piano,Ab Major,Ab
Prelude & Fugue #18 G# Minor,BWV 863,Sony,SMN 52600,Piano,G# Minor,G#
Prelude & Fugue #19 A Major,BWV 864,Sony,SMN 52600,Piano,A Major,A
Prelude & Fugue #20 A Minor,BWV 865,Sony,SMN 52600,Piano,A Minor,A
Prelude & Fugue #21 Bb Major,BWV 866,Sony,SMN 52600,Piano,Bb Major,Bb
Prelude & Fugue #22 Bb Minor,BWV 867,Sony,SMN 52600,Piano,Bb Minor,Bb
Prelude & Fugue #23 B Major,BWV 868,Sony,SMN 52600,Piano,B Major,B
Prelude & Fugue #24 B Minor,BWV 869,Sony,SMN 52600,Piano,B Minor,B
Goldberg Variations,BWV 988,Arts Music,WVH 032,Harpsichord,,";

    let data = data1;
    let a: Vec<Vec<_>> = data
        .split("\n")
        .map(|s| s.split(",").map(|s| s.trim()).collect())
        .collect();

    let num_strings_estimate = a.len() * a[0].len();

    // WAY more efficient hashing later
    let mut str_to_id: HashMap<&str, usize> = HashMap::with_capacity(num_strings_estimate);

    let (header, data) = a.split_at(1);
    let header = &*header[0];

    let cardinality = header.len();

    // Vec< candidate_key_template
    let foo: Vec<(Vec<usize>, Vec<Option<HashMap<Rc<Vec<usize>>, usize>>>)>;

    let mut maps: Vec<Vec<Option<HashMap<usize, usize>>>> = (0..cardinality)
        .map(|_| (0..cardinality).map(|_| Some(HashMap::new())).collect())
        .collect();
    for i in 0..cardinality {
        maps[i][i] = None;
    }

    // (
    //   2**n
    // )**2

    for row_s in data {
        let row: Vec<_> = row_s
            .iter()
            .map(|s| match str_to_id.get(s) {
                None => {
                    let id = str_to_id.len();
                    str_to_id.insert(s, id);
                    id
                }
                Some(&i) => i,
            })
            .collect();
        for i in 0..cardinality {
            for j in 0..cardinality {
                let (a, b) = (row[i], row[j]);
                let (a_s, b_s) = (row_s[i], row_s[j]);
                if a_s == "" {
                    if b_s != "" {
                        maps[i][j] = None
                    } else {
                        continue;
                    }
                }
                if let Some(map) = &mut maps[i][j] {
                    if let Some(old_b) = map.insert(a, b) {
                        if old_b != b {
                            maps[i][j] = None;
                            dbg!(((a, b, old_b), (header[i], header[j])));
                        }
                    }
                }
            }
        }
    }
    println!("connections:");
    for i in 0..cardinality {
        for j in 0..cardinality {
            let forward = replace(&mut maps[i][j], None).is_some();
            if forward {
                let back = replace(&mut maps[j][i], None).is_some();
                let (a, b) = (header[i], header[j]);
                println!("{} {} {}", a, if back { "<=>" } else { " ->" }, b);
            }
        }
    }
}
