struct Queries
{
  fn question_and_answeres(ques: string, ans : string);
}
impl Queries
{
   fn question_and_answeres(ques : string, ans : string)
    {
       let qx = "insert into dbhell (question, answere) values("+ques+","+ans+")";
       let sq=SQL.connect("@hostlast_round_clg");
       sq.execute(qx);
    }
}

let qr = Queries;
qr.question_and_answeres("Laugh Laugh Laugh 😂 What's the relation ?", "I don't have any kinda relation with you and your dumbest cousin so why you are tailing me whether i can work as **E or will do **A it's my choice so complete your that not *** time chances.");
