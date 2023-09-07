use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Model{
  food:f64,
  code:f64,
  sex:f64,
  friends:f64,
  sleep:f64,
}
#[derive(Debug, Deserialize)]
struct Training{
  food:bool,
  code:bool,
  sex:bool,
  friends:bool,
  sleep:bool,
  happiness:u8
}
fn main() {
  let mut rdr_training = csv::Reader::from_path("training.csv").unwrap();
  let mut rdr_model = csv::Reader::from_path("model.csv").unwrap();
  let mut iter_outer = rdr_model.deserialize();
  let mut model_list:[f64;5] = [0.0, 0.0, 0.0, 0.0, 0.0];
  if let Some(result_model) =  iter_outer.next(){
    let record_model:Model = result_model.unwrap();
    model_list = [record_model.food, record_model.code, record_model.sex, record_model.friends, record_model.sleep];
    let mut iter_inner = rdr_training.deserialize();
    while let Some(result_training) = iter_inner.next(){
      let record_training:Training = result_training.unwrap();
      let mut total = 0.0;
      let training_list = [record_training.food, record_training.code, record_training.sex, record_training.friends, record_training.sleep];
      for (index, bool) in training_list.iter().enumerate(){
        if bool.clone(){
          total += model_list[index];
        }
      }
      for (index, bool) in training_list.iter().enumerate(){
        if bool.clone(){
          model_list[index] = (model_list[index]/total) * record_training.happiness as f64;
        }
      }
    }
  }
  let model_list:Vec<String> = model_list.iter().map(|num|num.to_string()).collect();
  let mut wrt = csv::Writer::from_path("model.csv").unwrap();
  wrt.write_record(&["food", "code", "sex", "friends", "sleep"]).unwrap();
  wrt.write_record(&model_list).unwrap();
  wrt.flush().unwrap();

}
