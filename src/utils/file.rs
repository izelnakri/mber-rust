pub fn format_time_passed(time_passed: u128) -> String {
    return time_passed.to_string().as_str().to_owned() + "ms";
}

pub fn format_size<'a>(size_in_bytes: usize) -> String {
    if size_in_bytes > 999994 {
        return format!("{:.2} MB", (size_in_bytes as f64 / 1000000.0));
    }

    return format!("{:.2} kB", (size_in_bytes as f64 / 1000.0));
}

// pub fn report_file(file_path) -> HashMap<String, String> {

// }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_time_passed_turns_number_into_ms() {
        assert_eq!(format_time_passed(14230), "14230ms");
        assert_eq!(format_time_passed(3000), "3000ms");
        assert_eq!(format_time_passed(288), "288ms");
        assert_eq!(format_time_passed(55), "55ms");
        assert_eq!(format_time_passed(9), "9ms");
        assert_eq!(format_time_passed(0), "0ms");
    }

    #[test]
    fn format_size_returns_right_value_for_files_less_than_a_megabyte() {
        assert_eq!(format_size(0), "0.00 kB");
        assert_eq!(format_size(9), "0.01 kB");
        assert_eq!(format_size(23), "0.02 kB");
        assert_eq!(format_size(874), "0.87 kB");
        assert_eq!(format_size(1049), "1.05 kB");
        assert_eq!(format_size(1056), "1.06 kB");
        assert_eq!(format_size(75551), "75.55 kB");
        assert_eq!(format_size(75556), "75.56 kB");
        assert_eq!(format_size(99999), "100.00 kB");
        assert_eq!(format_size(109999), "110.00 kB");
        assert_eq!(format_size(999994), "999.99 kB");
        assert_eq!(format_size(999995), "1.00 MB");
        assert_eq!(format_size(999999), "1.00 MB");
    }

    #[test]
    fn format_size_returns_right_value_for_files_more_than_or_equal_to_a_megabyte() {
        assert_eq!(format_size(1000000), "1.00 MB");
        assert_eq!(format_size(1009000), "1.01 MB");
        assert_eq!(format_size(1656000), "1.66 MB");
        assert_eq!(format_size(2654000), "2.65 MB");
        assert_eq!(format_size(7611000), "7.61 MB");
        assert_eq!(format_size(12656000), "12.66 MB");
        assert_eq!(format_size(34656000), "34.66 MB");
        assert_eq!(format_size(99990012), "99.99 MB");
        assert_eq!(format_size(99999012), "100.00 MB");
        assert_eq!(format_size(111999012), "112.00 MB");
    }
}

// export function reportFile(filePath) {
//   return new Promise(async (resolve) => {
//     const fileBuffer = await fs.readFile(filePath);

//     return gzipAsync(fileBuffer).then((gzipBuffer) => {
//       const fileObject = {
//         fileName: stripProcessCWD(filePath),
//         size: fileBuffer.length,
//         gzipSize: gzipBuffer.length
//       }

//       console.log(
//         chalk.blue(` - ${fileObject.fileName}:`),
//         chalk.yellow(format_size(fileObject.size)),
//         chalk.green(`[${format_size(fileObject.gzipSize)} gzipped]`)
//       );

//       resolve(fileObject);
//     }).catch((error) => console.log('error', error));
//   });
// }
