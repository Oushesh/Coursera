## Serverless LLM apps with Amazon Bedrock
    Why use Serverless? 
    With serverless you dont need to manage Infrastructure Resources,  GPU scaling up and down with Kubernetes or helm. Cost

## Server Based System
    With server based system you need to manage Infrastructure Resources,  GPU scaling up and down with Kubernetes or helm. Cost.

## Serverless: 
   Files arrive in S3, AWS Transcribe similar to OpenAI Whisper
   or Facebook Sealess4MT. Event driven pipeline architecture.


## Your first generations with Amazon Bedrock.
   Portability of AWS Bedrock:
   You can access all of the most important Foundation models 
   from AWS and still make a good choice.

   Example: use boto3 from aws
   import boto3

   bedrock_runtime = boto3.client("bedrock-runtime",region_name=")
   
   assert (isinstance(prompt,str))
   kwargs = {
    "modelId": "amazon.titan-text-lite-v1",
    "contentType": "application/json",
    "accept": "*/*",
    "body": json.dumps(
        {
            "inputText": prompt
        }
    )
    }
   response =  bedrock_runtime.invote(**kwargs) #advantage to pass as dictionary. 

   response_body = json.loads(response.get("body").read())

   #Response_body is json
    Similar to openai


## Task 2:
   Audio transcription: Prompt Engineering. AWS Whisper
   AWS Bedrock: 
   
## Conclusion: AWS Transcribe is crazy fast. Need to check the quality vs openai whisper.


## Event Driven Architecture: 
   * Check the jupyter notebook.

## Serverless LLM apps with Amazon Bedrock.
   * Good to build Agents. Third party services.
   * AWS Bedrock Lambda Functions. all aws Bedrock --> RAG (Retrieval Augmented Generation)
    
##  Serverless AWS vs Predibase
    * Serverless AWS is much more expensive but good for big enterprises whereas Predibase will save you millions of dollars.

    * 