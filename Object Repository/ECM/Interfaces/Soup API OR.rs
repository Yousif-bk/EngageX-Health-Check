<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Soup API OR</name>
   <tag></tag>
   <elementGuidId>91985eb9-85f3-40b4-9844-00f4fd326d4b</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>true</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <katalonVersion>10.1.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <path></path>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:sms=&quot;http://com/comviva/ngage/ws/sms&quot; xmlns:sms1=&quot;http://sms.soap.ngage.comviva.com&quot;>
   &lt;soapenv:Header>
      &lt;sms:TransactionID>&lt;/sms:TransactionID>
   &lt;/soapenv:Header>
  &lt;soapenv:Body>
      &lt;sms:SMSSubmitRequest>
         &lt;sms:req>
            &lt;sms1:CampaignName>Campaign_Name&lt;/sms1:CampaignName>
            &lt;!--Optional:-->
            &lt;sms1:CampaignDesc>soap interface&lt;/sms1:CampaignDesc>
            &lt;sms1:Sender>
               &lt;sms1:Username>aabaker&lt;/sms1:Username>
               &lt;sms1:Password>Yousif@123&lt;/sms1:Password>
               &lt;!--Optional:-->
               &lt;sms1:Address>engageX&lt;/sms1:Address>
            &lt;/sms1:Sender>
            &lt;!--Optional:-->
            &lt;sms1:ProtocolId>&lt;/sms1:ProtocolId>
            &lt;!--Optional:-->
            &lt;sms1:CampaignCategory>4.5&lt;/sms1:CampaignCategory>
            &lt;!--Optional:-->
            &lt;sms1:PromotionalCategory>Campaign&lt;/sms1:PromotionalCategory>
            &lt;!--Optional:-->
            &lt;sms1:ContentType>3.2&lt;/sms1:ContentType>
            &lt;!--Optional:-->
            &lt;sms1:CallBackURL>&lt;/sms1:CallBackURL>
            &lt;!--Optional:-->
            &lt;sms1:ScheduledDeliveryDateTime>&lt;/sms1:ScheduledDeliveryDateTime>
            &lt;!--Optional:-->
            &lt;!--Optional:-->
            &lt;sms1:DeliveryReport>&lt;/sms1:DeliveryReport>
            &lt;!--Optional:-->
            &lt;sms1:ServiceName>&lt;/sms1:ServiceName>
            &lt;!--Optional:-->
            &lt;sms1:PriceTag>&lt;/sms1:PriceTag>
            &lt;sms1:JobType>
               &lt;!--Optional:-->
               &lt;sms1:Simple>
                  &lt;sms1:MsgDetails>
                     &lt;sms1:ShortMessage>EnageX Health Check: ECM Soup API Test&lt;/sms1:ShortMessage>
                  &lt;/sms1:MsgDetails>
                  &lt;sms1:Recipient>
                     &lt;!--Optional:-->
                     &lt;sms1:Number>971554573936&lt;/sms1:Number>
                     &lt;!--Optional:-->
                     &lt;sms1:DistributionList>&lt;/sms1:DistributionList>
                     &lt;!--Optional:-->
                     &lt;sms1:SegmentList>&lt;/sms1:SegmentList>
                     &lt;!--Optional:-->
                    &lt;sms1:Contact>&lt;/sms1:Contact>
                     &lt;!--Optional:-->
                     &lt;sms1:ContactGroup>&lt;/sms1:ContactGroup>
                     &lt;!--Optional:-->
                     &lt;sms1:FileURL>&lt;/sms1:FileURL>
                  &lt;/sms1:Recipient>
               &lt;/sms1:Simple>
               &lt;!--Optional:-->
               &lt;sms1:File>
                  &lt;sms1:DataFileURL>&lt;/sms1:DataFileURL>
               &lt;/sms1:File>
               &lt;!--Optional:-->
               &lt;sms1:Placeholders>
                  &lt;sms1:MsgDetails>
                     &lt;sms1:ShortMessage>&lt;/sms1:ShortMessage>
                  &lt;/sms1:MsgDetails>
                  &lt;sms1:Recipient>
                     &lt;sms1:DataFileURL>&lt;/sms1:DataFileURL>
                  &lt;/sms1:Recipient>
               &lt;/sms1:Placeholders>
            &lt;/sms1:JobType>
         &lt;/sms:req>
      &lt;/sms:SMSSubmitRequest>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>http://smartmessaging.etisalat.ae:9097/ws/sms</soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress>http://smartmessaging.etisalat.ae:9097/ws/sms</wsdlAddress>
</WebServiceRequestEntity>
