use super::html::Writer;
use super::problems::Problem;
use super::solutions;
use git2::Tree;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn make_solution_map(tree: &Tree) -> HashMap<String, Vec<String>> {
    let mut result = HashMap::<String, Vec<String>>::new();

    solutions::get(tree, |problem_id, solution_id| {
        result
            .entry(problem_id.to_string())
            .or_default()
            .push(solution_id.to_string());
    });

    result
}

fn write_hyper_link(writer: &mut Writer, href: &str, text: &str) {
    writer.element("a", &[("href", href)], |w| w.text(text));
}

fn write_problem_link(writer: &mut Writer, problem: &Problem) {
    write_hyper_link(
        writer,
        &format!("https://leetcode.com/problems/{}/", problem.stat.title_slug),
        &problem.stat.title,
    );
}

fn write_solution_link(writer: &mut Writer, problem_id: &str, solution: &str) {
    write_hyper_link(
        writer,
        &format!(
            "https://github.com/EFanZh/LeetCode/blob/master/src/problem_{}/{}.rs",
            problem_id, solution
        ),
        solution,
    );
}

fn write_difficulty(writer: &mut Writer, level: u8) {
    for _ in 0..level {
        writer.text("★");
    }
}

pub fn generate<P: AsRef<Path>>(problems: &[Problem], tree: &Tree, progress_chart: &str, output: P) {
    const TITLE: &str = "LeetCode Progress Report";
    let solution_map = make_solution_map(tree);
    let mut result = String::from("<!DOCTYPE html>\n");
    let mut html_writer = Writer::on(&mut result);

    html_writer.element("html", &[("lang", "en")], |w| {
        w.element("head", &[], |w| {
            w.empty_element("meta", &[("charset", "utf-8")]);
            w.element("title", &[], |w| w.text(TITLE));
            w.element("style", &[], |w| {
                w.raw(
                    r#"h1,h2 { text-align: center; }
figure { display: flex; justify-content: center; }
.detail { border-collapse: collapse; }
.detail>*>tr>* { padding: 0.125em 0.25em; text-align: left; }
.detail>*>tr>*:nth-child(1) { text-align: center; }
.detail>*>tr>*:nth-child(2) { text-align: right; }
.detail>thead>tr>th { background: white; position: sticky; top: 0; z-index: 1; }
.detail>tbody>tr:nth-child(odd) { background: #eee; }
.detail>tbody>tr>td>ul { margin: 0; padding: 0; list-style-type: none; }
.not-done>td { opacity: 0.382; }"#,
                );
            });
        });
        w.element("body", &[], |w| {
            w.element("h1", &[], |w| w.text(TITLE));
            w.element("div", &[("style", "text-align: center;")], |w| {
                write_hyper_link(w, "https://github.com/EFanZh/LeetCode", "Source code");
            });
            w.element("h2", &[], |w| w.text("Progress Chart"));
            w.element("figure", &[], |w| {
                w.empty_element("img", &[("src", progress_chart), ("alt", "Progress Chart")]);
            });
            w.element("h2", &[], |w| w.text("Detail"));
            w.element("figure", &[], |w| {
                w.element("table", &[("class", "detail")], |w| {
                    w.element("thead", &[], |w| {
                        w.element("tr", &[], |w| {
                            w.element("th", &[], |w| w.text("Done"));
                            w.element("th", &[], |w| w.text("ID"));
                            w.element("th", &[], |w| w.text("Title"));
                            w.element("th", &[], |w| w.text("Difficulty"));
                            w.element("th", &[], |w| w.text("Solutions"));
                        });
                    });
                    w.element("tbody", &[], |w| {
                        for problem in problems {
                            if let Some(solution_list) = solution_map.get(&problem.get_id()) {
                                w.element("tr", &[], |w| {
                                    w.element("td", &[], |w| w.text("✔"));
                                    w.element("td", &[], |w| {
                                        w.text(&problem.stat.frontend_question_id.to_string());
                                    });
                                    w.element("td", &[], |w| write_problem_link(w, problem));
                                    w.element("td", &[], |w| write_difficulty(w, problem.difficulty.level));
                                    w.element("td", &[], |w| {
                                        w.element("ul", &[], |w| {
                                            let problem_id = problem.get_id();

                                            for solution in solution_list {
                                                w.element("li", &[], |w| {
                                                    write_solution_link(w, &problem_id, solution);
                                                });
                                            }
                                        });
                                    });
                                });
                            } else {
                                w.element("tr", &[("class", "not-done")], |w| {
                                    w.element("td", &[], |_| {});
                                    w.element("td", &[], |w| w.text(&problem.stat.frontend_question_id.to_string()));
                                    w.element("td", &[], |w| write_problem_link(w, problem));
                                    w.element("td", &[], |w| write_difficulty(w, problem.difficulty.level));
                                    w.element("td", &[], |_| {});
                                });
                            }
                        }
                    });
                });
            });
        });
    });

    result.push('\n');

    fs::write(output, result).unwrap();
}
