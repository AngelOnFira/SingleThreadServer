#[derive(Debug)]
pub enum Method {
    GET {
        file : String 
    },
    HEAD {
        file : String
    },
    POST {
        body : String,
        content_type : String
    },
    PUT,
    DELETE,
    CONNECT {
        URL : String
    },
    OPTIONS {
        URL : String
    },
    TRACE,
    PATCH
}

#[derive(Debug)]
pub enum ParserError {
    InvalidMethod,
    FileDoesNotExist,
    NotImplemented,
}

pub mod HTTPBody {
    trait VariantName {
        fn get_variant(&self) -> String;
    }

    //https://www.geeksforgeeks.org/http-headers-content-type/
    pub enum ContentType {
        Application {value : Value::Application},
        Audio {value : Value::Audio},
        Image {value : Value::Image},
        Multipart {value : Value::Multipart},
        Text {value : Value::Text},
        Video {value : Value::Video},
    }

    pub mod Value{
        pub enum Application {
            EDI_X12,
            EDIFACT,
            javascript,
            octet_stream,
            ogg,
            pdf,
            xhtml_xml,
            x_shockwave_flash,
            json,
            ld_json,
            xml,
            zip,
            x_www_form_urlencoded,
        }
        impl super::VariantName for Application {
            fn get_variant(&self) -> String{
                match &self {
                    Application::EDI_X12 => return String::from("EDI-X12"),
                    Application::EDIFACT => return String::from("EDIFACT"),
                    Application::javascript => return String::from("javascript"),
                    Application::octet_stream => return String::from("octet-stream"),
                    Application::ogg => return String::from("ogg"),
                    Application::pdf => return String::from("pdf"),
                    Application::xhtml_xml => return String::from("xhtml+xml"),
                    Application::x_shockwave_flash => return String::from("x-shockwave-flash"),
                    Application::json => return String::from("json"),
                    Application::ld_json => return String::from("ld+json"),
                    Application::xml => return String::from("xml"),
                    Application::zip => return String::from("zip"),
                    Application::x_www_form_urlencoded => return String::from("x-www-form-urlencoded"),
                    _ => panic!("Invalid variant type"),
                }
            }
        }

        pub enum Audio {
            mpeg,
            x_ms_wma,
            vnd_rn_realaudio,
            x_wav,
        }
        impl super::VariantName for Audio {
            fn get_variant(&self) -> String{
                match &self {
                    Audio::mpeg => return String::from("mpeg"),
                    Audio::x_ms_wma => return String::from("x-ms-wma"),
                    Audio::vnd_rn_realaudio => return String::from("vnd.rn-realaudio"),
                    Audio::x_wav => return String::from("x-wav"),
                    _ => panic!("Invalid variant type"),
                }
            }
        }

        pub enum Image {
            gif,
            jpeg,
            png,
            tiff,
            vnd_microsoft_icon,
            x_icon,
            vnd_djvu,
            svg_xml,
        }
        impl super::VariantName for Image {
            fn get_variant(&self) -> String{
                match &self {
                    Image::gif => return String::from("gif"),
                    Image::jpeg => return String::from("jpeg"),
                    Image::png => return String::from("png"),
                    Image::tiff => return String::from("tiff"),
                    Image::vnd_microsoft_icon => return String::from("vnd.microsoft.icon"),
                    Image::x_icon => return String::from("x-icon"),
                    Image::vnd_djvu => return String::from("vnd.djvu"),
                    Image::svg_xml => return String::from("svg+xml"),
                    _ => panic!("Invalid variant type"),
                }
            }
        }

        pub enum Multipart {
            mixed,
            alternative,
            related,
            form_data {boundary : String},
        }
        impl super::VariantName for Multipart {
            fn get_variant(&self) -> String{
                match &self {
                    Multipart::mixed => String::from("mixed"),
                    Multipart::alternative => String::from("alternative"),
                    Multipart::related => String::from("related"),
                    Multipart::form_data { boundary } => String::from("form-data"),
                    _ => panic!("Invalid variant type"),
                }
            }
        }

        pub enum Text {
            css,
            csv,
            html,
            javascript,
            plain,
            xml,
        }
        impl super::VariantName for Text {
            fn get_variant(&self) -> String{
                match &self {
                    Text::css => String::from("css"),
                    Text::csv => String::from("csv"),
                    Text::html => String::from("html"),
                    Text::javascript => String::from("javascript"),
                    Text::plain => String::from("plain"),
                    Text::xml => String::from("xml"),
                    _ => panic!("Invalid variant type"),
                }
            }
        }

        pub enum Video {
            mpeg,
            mp4,
            quicktime,
            x_ms_wmv,
            x_msvideo,
            x_flv,
            webm,
        }
        impl super::VariantName for Video {
            fn get_variant(&self) -> String{
                match &self {
                    Video::mpeg => String::from("mpeg"),
                    Video::mp4 => String::from("mp4"),
                    Video::quicktime => String::from("quicktime"),
                    Video::x_ms_wmv => String::from("x-ms-wmv"),
                    Video::x_msvideo => String::from("x-msvideo"),
                    Video::x_flv => String::from("x-flv"),
                    Video::webm => String::from("webm"),
                    _ => panic!("Invalid variant type"),
                }
            }
        }

    }
    impl VariantName for ContentType {
        fn get_variant(&self) -> String{
            match &self {
                ContentType::Application { value } => return String::from("application"),
                ContentType::Audio { value } => return String::from("audio"),
                ContentType::Image { value } => return String::from("image"),
                ContentType::Multipart { value } => return String::from("multipart"),
                ContentType::Text { value } => return String::from("text"),
                ContentType::Video { value } => return String::from("video"),
                _ => panic!("Invalid variant type"),
            }
        }
    }
    impl ToString for ContentType {
        fn to_string(&self) -> String {
            match &self {
                ContentType::Application { value }  => {
                    return format!("{}/{}", &self.get_variant(), value.get_variant())
                },
                ContentType::Audio { value }   => {
                    return format!("{}/{}", &self.get_variant(), value.get_variant())
                },
                 ContentType::Image { value }   => {
                    return format!("{}/{}", &self.get_variant(), value.get_variant())
                },
                 ContentType::Multipart { value }   => {
                    return format!("{}/{}", &self.get_variant(), value.get_variant())
                },
                 ContentType::Text { value }   => {
                    return format!("{}/{}", &self.get_variant(), value.get_variant())
                },
                 ContentType::Video { value } => {
                    return format!("{}/{}", &self.get_variant(), value.get_variant())
                },
                _ => panic!("Not implemented variant"),
            }
            
        }
    }
}

impl Method {
    pub fn new(request_data : [u8; 1024]) -> Result<Method, ParserError> {
        let request : String = String::from_utf8_lossy(&request_data[..]).to_string();
        let request : Vec<&str> = request.split("\n").collect();

        if request.len() == 0 {
            return Result::Err(ParserError::InvalidMethod);
        }

        match Method::get_type(&request[0]){
            Some(method) => {
                let method : String = method.to_uppercase();

                match method.as_str() {
                    "GET"       => return Method::new_GET(request),
                    "HEAD"      => return Method::new_HEAD(request),
                    "POST"      => return Method::new_POST(request),
                    "PUT"       => return Method::new_PUT(request),
                    "DELETE"    => return Method::new_DELETE(request),
                    "CONNECT"   => return Method::new_CONNECT(request),
                    "OPTIONS"   => return Method::new_OPTIONS(request),
                    "TRACE"     => return Method::new_TRACE(request),
                    "PATCH"     => return Method::new_PATCH(request),
                    _           => return Result::Err(ParserError::InvalidMethod),
                }
            },
            None => {
                return Result::Err(ParserError::InvalidMethod);
            },
        }
    }

    fn new_GET(request : Vec<&str>) -> Result<Method, ParserError> {
        let start_line : Vec<&str> = request[0].clone().split_whitespace().collect();

        if start_line.len() < 3 {
            return Result::Err(ParserError::InvalidMethod);
        }

        let file_name : &str = start_line[1];

        Result::Ok(Method::GET{ file : file_name.to_string()})   
    }

    fn new_HEAD(request : Vec<&str>) -> Result<Method, ParserError> {
        let start_line : Vec<&str> = request[0].clone().split_whitespace().collect();

        if start_line.len() < 3 {
            return Result::Err(ParserError::InvalidMethod);
        }

        let file_name : &str = start_line[1];

        Result::Ok(Method::HEAD{ file : file_name.to_string()})
    }

    fn new_POST(request : Vec<&str>) -> Result<Method, ParserError> {
        Result::Err(ParserError::NotImplemented)
    }

    fn new_PUT(request : Vec<&str>) -> Result<Method, ParserError> {
        Result::Err(ParserError::NotImplemented)
    }

    fn new_DELETE(request : Vec<&str>) -> Result<Method, ParserError> {
        Result::Err(ParserError::NotImplemented)
    }

    fn new_CONNECT(request : Vec<&str>) -> Result<Method, ParserError> {
        let start_line : Vec<&str> = request[0].clone().split_whitespace().collect();

        if start_line.len() < 3 {
            return Result::Err(ParserError::InvalidMethod);
        }

        let URL_name : &str = start_line[1];

        Result::Ok(Method::CONNECT{ URL : URL_name.to_string()})
    }

    fn new_OPTIONS(request : Vec<&str>) -> Result<Method, ParserError> {
        let start_line : Vec<&str> = request[0].clone().split_whitespace().collect();

        if start_line.len() < 3 {
            return Result::Err(ParserError::InvalidMethod);
        }

        let URL_name : &str = start_line[1];

        Result::Ok(Method::OPTIONS{ URL : URL_name.to_string()})
    }

    fn new_TRACE(request : Vec<&str>) -> Result<Method, ParserError> {
        Result::Err(ParserError::NotImplemented)
    }

    fn new_PATCH(request : Vec<&str>) -> Result<Method, ParserError> {
        Result::Err(ParserError::NotImplemented)
    }

    fn get_type<'a>(request: &'a str) -> Option<&'a str> {
        request.split_whitespace().next()
    }

    fn get_start_line<'a>(start_line: &'a str) -> Result<(&'a str, &'a str, &'a str), ParserError> {
        let mut start_line = start_line.split_whitespace();
        
        let method : &str = start_line.next().ok_or(ParserError::InvalidMethod)?;
        let target : &str = start_line.next().ok_or(ParserError::InvalidMethod)?;
        let version : &str = start_line.next().ok_or(ParserError::InvalidMethod)?;

        Result::Ok((method, target, version))
    }
#[cfg(test)]
mod tests {
    mod HTTP_Body_Enum_Test {
        use crate::HTTPRequest::HTTPBody::*;
        
        #[test]
        fn content_type_application_test(){
            {
                let actual : ContentType = ContentType::Application {
                    value : Value::Application::EDI_X12
                };
                assert_eq!(actual.to_string(), "application/EDI-X12");
            }

            {
                let actual : ContentType = ContentType::Application {
                    value : Value::Application::EDIFACT
                };
                assert_eq!(actual.to_string(), "application/EDIFACT");
            }

            {
                let actual : ContentType = ContentType::Application {
                    value : Value::Application::javascript
                };
                assert_eq!(actual.to_string(), "application/javascript");
            }

            {
                let actual : ContentType = ContentType::Application {
                    value : Value::Application::octet_stream
                };
                assert_eq!(actual.to_string(), "application/octet-stream");
            }

            {
                let actual : ContentType = ContentType::Application {
                    value : Value::Application::ogg
                };
                assert_eq!(actual.to_string(), "application/ogg");
            }

            {
                let actual : ContentType = ContentType::Application {
                    value : Value::Application::pdf
                };
                assert_eq!(actual.to_string(), "application/pdf");
            }

            {
                let actual : ContentType = ContentType::Application {
                    value : Value::Application::xhtml_xml
                };
                assert_eq!(actual.to_string(), "application/xhtml+xml");
            }

            {
                let actual : ContentType = ContentType::Application {
                    value : Value::Application::x_shockwave_flash
                };
                assert_eq!(actual.to_string(), "application/x-shockwave-flash");
            }

            {
                let actual : ContentType = ContentType::Application {
                    value : Value::Application::json
                };
                assert_eq!(actual.to_string(), "application/json");
            }

            {
                let actual : ContentType = ContentType::Application {
                    value : Value::Application::ld_json
                };
                assert_eq!(actual.to_string(), "application/ld+json");
            }

            {
                let actual : ContentType = ContentType::Application {
                    value : Value::Application::xml
                };
                assert_eq!(actual.to_string(), "application/xml");
            }

            {
                let actual : ContentType = ContentType::Application {
                    value : Value::Application::zip
                };
                assert_eq!(actual.to_string(), "application/zip");
            }

            {
                let actual : ContentType = ContentType::Application {
                    value : Value::Application::x_www_form_urlencoded
                };
                assert_eq!(actual.to_string(), "application/x-www-form-urlencoded");
            }
        }

        #[test]
        fn content_type_audio_test(){

        }

        #[test]
        fn content_type_image_test(){
            
        }

        #[test]
        fn content_type_multipart_test(){
            
        }

        #[test]
        fn content_type_text_test(){
            
        }

        #[test]
        fn content_type_video_test(){
            
        }
    }
    mod HTTP_Request_Parse_Test {
        //get test
        //head test
        //post test
        //put test
        //delete test
        //connect test
        //options test
        //trace test
        //patch test
    }
}